use crate::f32_3::{dd_f32_3, mltply_f32_3, sbtr_f32_3};

pub fn f32_3_dots_collinear(
    point_1: [f32; 3],
    point_2: [f32; 3],
    points_number: u32,
) -> Vec<[f32; 3]> {
    let mut planes_points = vec![];
    let points_diff = sbtr_f32_3(point_2, point_1);

    for i in 0..points_number {
        planes_points.push(dd_f32_3(
            mltply_f32_3(points_diff, (i as f32) / (points_number as f32)),
            point_1,
        ));
    }

    return planes_points;
}

pub fn rotational_distance_function_sine(
    x: f32,
    c_arg: Vec<f32>, // f_const: f32,
                     // f_multiplier: f32,
                     // x_const: f32,
                     // x_multiplier: f32,
) -> f32 {
    return c_arg[0] + c_arg[1] * (c_arg[2] + x * c_arg[3]).sin();
}

pub fn spherical_progress(points_diff: f32, pln: f32, planes_points: f32) -> f32 {
    let d = (points_diff / 2.0) * (pln - planes_points / 2.0).abs() / (planes_points / 2.0);
    let f = ((points_diff / 2.0).powi(2) - d.powi(2)).sqrt();
    return f;
}
