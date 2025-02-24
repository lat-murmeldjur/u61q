#![allow(warnings)] // not today, erosion
use rand::Rng;

use crate::f32_3::{
    angle_360_of, angular_difference, average_f32_3, dd_f32_3, find_points_normal, gen_f32_3,
    gen_rthgnl_f32_3, mltply_f32_3, nrmlz_f32_3, sbtr_f32_3, vector_length,
};
use crate::positions::{create_points_on_cross_section, sort_positions_by_angle, Normal, Position};
use crate::shapes::{f32_3_dots_collinear, rotational_distance_function_sine, spherical_progress};
use crate::u_modular::{modular_difference_in_range, modular_offset_in_range};

#[derive(Debug)]
pub struct Magma {
    positions: Vec<Position>,
    indices: Vec<u32>,
}

#[derive(Debug)]
pub struct Stone {
    pub positions: Vec<Position>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
}

pub fn magma(flow: u32, scale: f32) -> Magma {
    let mut rng = rand::thread_rng();

    let mut lava_flow = Magma {
        positions: vec![],
        indices: vec![],
    };

    let base = scale;
    let mut cbase = -2.5 * scale;
    for i in 1..=flow {
        lava_flow.positions.push(Position {
            position: gen_f32_3(cbase, base, &mut rng),
        });
        cbase = cbase + 5.0 * base;

        // randomize graph edges
        if i > 1 {
            lava_flow
                .indices
                .push(rng.gen_range(0..lava_flow.positions.len() - 1) as u32);
            lava_flow.indices.push(i - 1);
        }
    }

    // println!("flow: {:#?}", lava_flow);

    return lava_flow;
}

pub fn petrify(flow: Magma) -> Stone {
    if flow.positions.len() > 2 {
        return petrify_flow(flow);
    };

    let mut stone = Stone {
        positions: vec![],
        normals: vec![],
        indices: vec![],
    };

    let mut rng = rand::thread_rng();
    let points_diff = sbtr_f32_3(flow.positions[1].position, flow.positions[0].position);
    let planes_normal: [f32; 3] = nrmlz_f32_3(points_diff);
    let planes_number = 30; // rng.gen_range(40..42);
    let f__max_points = 30.0; // rng.gen_range(40.0..42.0);
    let max_points = f__max_points;
    let mut points_of_plane: u32 = 3;
    let reference_orthogonal = gen_rthgnl_f32_3(planes_normal, &mut rng);
    let mut pln = 0;

    let planes_points = f32_3_dots_collinear(
        flow.positions[0].position,
        flow.positions[1].position,
        planes_number,
    );

    let mut previous_plane: [u32; 3] = [0, 0, 0]; // plane number, beginning position, ending position

    for plane_point in planes_points.iter() {
        // println!("plane: {:#?}", plane_point);
        // vector_length(points_diff) / 2.0 * (PI * pln as f32 / planes_points.len() as f32).sin()

        let rotational_arguments_vector = vec![
            spherical_progress(
                vector_length(points_diff),
                pln as f32,
                planes_number as f32 - 1.0,
            ),
            0.0,
            0.0,
            0.0,
        ]; // f_const: f32,
           // f_multiplier: f32,
           // x_const: f32,
           // x_multiplier: f32,

        let mut plane = Stone {
            positions: vec![],
            normals: vec![],
            indices: vec![],
        };

        plane.positions = create_points_on_cross_section(
            rotational_distance_function_sine,
            rotational_arguments_vector.clone(),
            reference_orthogonal,
            planes_normal,
            *plane_point,
            points_of_plane, //points_number
            &mut rng,
        );

        points_of_plane = plane.positions.len() as u32;

        sort_positions_by_angle(
            *plane_point,
            reference_orthogonal,
            planes_normal,
            &mut plane.positions,
        );

        let mut normal_origin = *plane_point;
        if pln == 0 {
            normal_origin = planes_points[1];
        }
        if pln == planes_number - 1 {
            normal_origin = planes_points[(planes_number - 2) as usize];
        }

        for i in 0..points_of_plane {
            stone.positions.push(Position {
                position: plane.positions[i as usize].position,
            });
            let normal = //nrmlz_f32_3(gen_f32_3(0.0, 1.0, &mut rng));
            // find_points_normal(plane.positions[(i as usize)].position, *plane_point);
            find_points_normal(plane.positions[i as usize].position, normal_origin);
            stone.normals.push(Normal { normal: normal });
        }

        if previous_plane[2] == 0 {
            stone.indices.push(0);
            stone.indices.push(1);
            stone.indices.push(2);
        } else {
            //            println!(
            //                "############# {} {}",
            //                previous_plane[1],
            //                previous_plane[2] - 1
            //            );
            //            println!(
            //                "############# {} {}",
            //                previous_plane[2],
            //                previous_plane[2] + points_of_plane - 1
            //            );
            find_indices_double_circle(
                //vertex_plane_one: [u32; 2],
                [previous_plane[1], previous_plane[2] - 1],
                //plane_one: [f32; 3],
                planes_points[previous_plane[0] as usize],
                //vertex_plane_two: [u32; 2],
                [previous_plane[2], previous_plane[2] + points_of_plane - 1],
                //plane_two: [f32; 3],
                *plane_point,
                //reference_orthogonal: [f32; 3],
                reference_orthogonal,
                //planes_normal: [f32;3],
                planes_normal,
                //&mut stone: Stone,
                &mut stone,
            );
        };
        if previous_plane[0] == planes_number - 2 {
            stone.indices.push((stone.positions.len() - 3) as u32);
            stone.indices.push((stone.positions.len() - 2) as u32);
            stone.indices.push((stone.positions.len() - 1) as u32);
        }

        // prepare next plane

        previous_plane[0] = pln;
        pln = pln + 1;
        previous_plane[1] = previous_plane[2];
        previous_plane[2] = previous_plane[2] + points_of_plane;

        if previous_plane[0] == planes_number - 2 {
            points_of_plane = 3;
        } else {
            points_of_plane =
                3 + spherical_progress(max_points, pln as f32, planes_number as f32 - 1.0) as u32;
        };
    }

    return stone;
}

pub fn petrify_flow(_flow: Magma) -> Stone {
    return Stone {
        positions: vec![],
        normals: vec![],
        indices: vec![],
    };
}

pub fn find_indices_double_circle(
    single_vertex_plane: [u32; 2],
    single_plane_point: [f32; 3],
    double_vertex_plane: [u32; 2],
    double_plane_point: [f32; 3],
    reference_orthogonal: [f32; 3],
    planes_normal: [f32; 3],
    stone: &mut Stone,
) {
    let mut index_set = false;
    let mut index_double_saved = 0;
    let mut index_single_saved = 0;

    let points_of_single_plane = single_vertex_plane[1] - single_vertex_plane[0] + 1;

    let points_of_double_plane = double_vertex_plane[1] - double_vertex_plane[0] + 1;

    let mut single_planes_points_average = [0.0, 0.0, 0.0];
    for i in single_vertex_plane[0]..=single_vertex_plane[1] {
        single_planes_points_average = dd_f32_3(
            single_planes_points_average,
            stone.positions[i as usize].position,
        );
    }

    single_planes_points_average = mltply_f32_3(
        single_planes_points_average,
        1.0 / (points_of_single_plane as f32),
    );

    let single_planes_points_center = sbtr_f32_3(single_planes_points_average, single_plane_point);

    let mut double_planes_points_average = [0.0, 0.0, 0.0];
    for i in double_vertex_plane[0]..=double_vertex_plane[1] {
        double_planes_points_average = dd_f32_3(
            double_planes_points_average,
            stone.positions[i as usize].position,
        );
    }

    double_planes_points_average = mltply_f32_3(
        double_planes_points_average,
        1.0 / (points_of_double_plane as f32),
    );

    let double_planes_points_center = sbtr_f32_3(double_planes_points_average, double_plane_point);

    let mut first_single_index = 0;
    let mut triangle_counter = 0;
    let mut a_min;
    let mut a_min_dex;
    let mut k;
    let mut pointlike = false;

    for i in double_vertex_plane[0]..=double_vertex_plane[1] + 1 {
        a_min = f32::MAX;
        a_min_dex = 0;
        k = i + 1;
        if k < double_vertex_plane[1] + 2 {
            if k > double_vertex_plane[1] {
                k = double_vertex_plane[0];
            }
            // take the points,
            // get their vector from the circle's average point
            // translate this distance to the plane point on the interplane-axis ("normal" points)
            // so the angles we get are circular
            // no matter if the circle points are not even around the interplane axis
            // they can be located far away in a single direction

            let po1 = sbtr_f32_3(
                stone.positions[i as usize].position,
                double_planes_points_center,
            );
            let po2 = sbtr_f32_3(
                stone.positions[k as usize].position,
                double_planes_points_center,
            );

            let center = double_plane_point;

            let nrml_point_1 = dd_f32_3(find_points_normal(center, po1), center);
            let nrml_point_2 = dd_f32_3(find_points_normal(center, po2), center);

            // get an average of the two neighboring "normal" points
            // and find the closest angled "normal" point in the other circle compared to reference orthogonal

            let average_point = average_f32_3(vec![nrml_point_1, nrml_point_2]);

            for j in single_vertex_plane[0]..=single_vertex_plane[1] {
                let po3 = sbtr_f32_3(
                    stone.positions[j as usize].position,
                    single_planes_points_center,
                );
                let nrml_point_3 = dd_f32_3(
                    find_points_normal(single_plane_point, po3),
                    single_plane_point,
                );

                let dist = angular_difference(
                    angle_360_of(
                        double_plane_point,
                        average_point,
                        reference_orthogonal,
                        planes_normal,
                    ),
                    angle_360_of(
                        single_plane_point,
                        nrml_point_3,
                        reference_orthogonal,
                        planes_normal,
                    ),
                );

                if dist < a_min {
                    a_min = dist;
                    a_min_dex = j;
                }
            }
            stone.indices.push(i);
            stone.indices.push(k);
            stone.indices.push(a_min_dex);
            triangle_counter = triangle_counter + 1;
        } else {
            a_min_dex = first_single_index;
            if a_min_dex == index_single_saved && triangle_counter == points_of_double_plane {
                pointlike = true;
            }
        }

        if index_set {
            let mut running_index = index_single_saved;

            let mut loop_for = modular_difference_in_range(
                index_single_saved,
                a_min_dex,
                single_vertex_plane[0],
                single_vertex_plane[1],
            );

            if pointlike {
                loop_for = single_vertex_plane[1] - single_vertex_plane[0] + 1;
            }

            for _l in 1..=loop_for {
                stone.indices.push(index_double_saved);
                stone.indices.push(running_index);
                stone.indices.push(modular_offset_in_range(
                    running_index,
                    1,
                    single_vertex_plane[0],
                    single_vertex_plane[1],
                ));

                triangle_counter = triangle_counter + 1;

                running_index = modular_offset_in_range(
                    running_index,
                    1,
                    single_vertex_plane[0],
                    single_vertex_plane[1],
                );
            }
        } else {
            first_single_index = a_min_dex;
        }

        index_set = true;
        index_double_saved = k;
        index_single_saved = a_min_dex;
    }

    // debug
    let mut debug = false;

    if points_of_single_plane + points_of_double_plane > triangle_counter {
        println!(
            "Added less triangles ({}) than necessary ({})",
            triangle_counter,
            points_of_single_plane + points_of_double_plane
        );

        debug = true;
    }
    if points_of_single_plane + points_of_double_plane < triangle_counter {
        println!(
            "Added *****more***** triangles ({}) than necessary ({})",
            triangle_counter,
            points_of_single_plane + points_of_double_plane
        );

        debug = true;
    }

    if debug {
        println!("Further Info 1{:#?}", single_vertex_plane,);

        println!("Further Info 2{:#?}", double_vertex_plane,);
    }
}
