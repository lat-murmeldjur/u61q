use crate::f32_3::{
    angle_360_of, dd_f32_3, gen_f32_3_unit_on_point_normal_plane, mltply_f32_3, sbtr_f32_3,
};
use rand::rngs::ThreadRng;

use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};
#[derive(BufferContents, Vertex, Debug, Clone, Copy)]
#[repr(C)]
pub struct Position {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
}

#[derive(BufferContents, Vertex, Debug, Clone, Copy)]
#[repr(C)]
pub struct Normal {
    #[format(R32G32B32_SFLOAT)]
    pub normal: [f32; 3],
}

pub fn create_points_on_cross_section(
    c: fn(f32, Vec<f32>) -> f32,
    c_arg: Vec<f32>,
    reference_orthogonal: [f32; 3],
    planes_normal: [f32; 3],
    plane_point: [f32; 3],
    points_number: u32,
    rng: &mut ThreadRng,
) -> Vec<Position> {
    let mut positions = vec![];
    for _i in 1..=points_number {
        let point = gen_f32_3_unit_on_point_normal_plane(planes_normal, plane_point, 1.0, rng);

        let angle = angle_360_of(plane_point, point, reference_orthogonal, planes_normal);

        let distance_multiplier = c(angle, c_arg.clone());

        let normal = sbtr_f32_3(point, plane_point);

        let point_translated = dd_f32_3(plane_point, mltply_f32_3(normal, distance_multiplier));

        positions.push(Position {
            position: point_translated,
        });
    }

    return positions;
}

pub fn positions_average(positions: &Vec<Position>) -> [f32; 3] {
    let mut planes_points_average = [0.0, 0.0, 0.0];
    for i in 0..positions.len() {
        planes_points_average = dd_f32_3(planes_points_average, positions[i].position);
    }

    return mltply_f32_3(planes_points_average, 1.0 / (positions.len() as f32));
}

pub fn sort_positions_by_angle(
    plane_point: [f32; 3],
    reference_orthogonal: [f32; 3],
    planes_normal: [f32; 3],
    positions: &mut Vec<Position>,
) {
    let planes_points_average = positions_average(positions);

    let planes_points_center = sbtr_f32_3(planes_points_average, plane_point);
    // order points on plane by angle

    positions.sort_by(|a, b| {
        angle_360_of(
            plane_point,
            sbtr_f32_3(a.position, planes_points_center),
            reference_orthogonal,
            planes_normal,
        )
        .total_cmp(&angle_360_of(
            plane_point,
            sbtr_f32_3(b.position, planes_points_center),
            reference_orthogonal,
            planes_normal,
        ))
    });
}

pub fn move_positions(positions: &mut Vec<Position>, move_by: [f32; 3]) {
    for i in 0..positions.len() {
        positions[i].position = dd_f32_3(positions[i].position, move_by);
    }
}
