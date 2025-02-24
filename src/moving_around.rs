use crate::f32_3::{
    dd_f32_3, dot_product, dstnc_f32_3, find_orthogonal_f32_3, find_points_normal, mltply_f32_3,
};

use crate::positions::Position;

// mod u_modular;
// use u_modular::{
//     modular_difference, modular_difference_in_range, modular_offset, modular_offset_in_range,
// };
pub fn move_forwards(
    view_point: &mut Position,
    center: &mut Position,
    _up_direction: &mut Position,
    rate: f32,
) {
    let direction = mltply_f32_3(
        find_points_normal(view_point.position, center.position),
        rate,
    );
    view_point.position = dd_f32_3(view_point.position, direction);
    center.position = dd_f32_3(center.position, direction);
}

pub fn move_sideways(
    view_point: &mut Position,
    center: &mut Position,
    up_direction: &mut Position,
    rate: f32,
) {
    let look_direction = find_points_normal(view_point.position, center.position);
    let orthogonal = find_orthogonal_f32_3(look_direction, up_direction.position);
    let direction = mltply_f32_3(find_points_normal([0.0, 0.0, 0.0], orthogonal), rate);
    view_point.position = dd_f32_3(view_point.position, direction);
    center.position = dd_f32_3(center.position, direction);
}

pub fn move_elevation(
    view_point: &mut Position,
    center: &mut Position,
    up_direction: &mut Position,
    rate: f32,
) {
    let look_direction = find_points_normal(view_point.position, center.position);
    let orthogonal = find_orthogonal_f32_3(look_direction, up_direction.position);
    let second_orthogonal = find_orthogonal_f32_3(look_direction, orthogonal);

    let direction = mltply_f32_3(find_points_normal([0.0, 0.0, 0.0], second_orthogonal), rate);
    view_point.position = dd_f32_3(view_point.position, direction);
    center.position = dd_f32_3(center.position, direction);
}

pub fn rotate_up(
    view_point: &mut Position,
    center: &mut Position,
    up_direction: &mut Position,
    rate: f32,
) {
    let sin_t = rate.sin();
    let cos_t = rate.cos();
    let look_direction = find_points_normal(view_point.position, center.position);

    let rodrigues_part_one = dd_f32_3(
        mltply_f32_3(up_direction.position, cos_t),
        mltply_f32_3(
            find_orthogonal_f32_3(look_direction, up_direction.position),
            sin_t,
        ),
    );
    let rodrigues_part_two = mltply_f32_3(
        look_direction,
        dot_product(look_direction, up_direction.position) * (1.0 - cos_t),
    );
    up_direction.position = dd_f32_3(rodrigues_part_one, rodrigues_part_two);
}

pub fn rotate_horizontal(
    view_point: &mut Position,
    center: &mut Position,
    up_direction: &mut Position,
    rate: f32,
) {
    let sin_t = rate.sin();
    let cos_t = rate.cos();
    let look_direction = find_points_normal(view_point.position, center.position);
    let look_distance = dstnc_f32_3(view_point.position, center.position);
    let orthogonal = find_orthogonal_f32_3(look_direction, up_direction.position);
    let second_orthogonal = find_orthogonal_f32_3(look_direction, orthogonal);
    up_direction.position = mltply_f32_3(second_orthogonal, -1.0);

    let rodrigues_part_one = dd_f32_3(
        mltply_f32_3(look_direction, cos_t),
        mltply_f32_3(
            find_orthogonal_f32_3(second_orthogonal, look_direction),
            sin_t,
        ),
    );
    let rodrigues_part_two = mltply_f32_3(
        second_orthogonal,
        dot_product(second_orthogonal, look_direction) * (1.0 - cos_t),
    );
    center.position = dd_f32_3(
        mltply_f32_3(
            dd_f32_3(rodrigues_part_one, rodrigues_part_two),
            look_distance,
        ),
        view_point.position,
    );
}

pub fn rotate_vertical(
    view_point: &mut Position,
    center: &mut Position,
    up_direction: &mut Position,
    rate: f32,
) {
    let sin_t = rate.sin();
    let cos_t = rate.cos();
    let look_direction = find_points_normal(view_point.position, center.position);
    let look_distance = dstnc_f32_3(view_point.position, center.position);
    let orthogonal = find_orthogonal_f32_3(look_direction, up_direction.position);
    up_direction.position = mltply_f32_3(find_orthogonal_f32_3(look_direction, orthogonal), -1.0);

    let rodrigues_part_one = dd_f32_3(
        mltply_f32_3(look_direction, cos_t),
        mltply_f32_3(find_orthogonal_f32_3(orthogonal, look_direction), sin_t),
    );
    let rodrigues_part_two = mltply_f32_3(
        orthogonal,
        dot_product(orthogonal, look_direction) * (1.0 - cos_t),
    );
    center.position = dd_f32_3(
        mltply_f32_3(
            dd_f32_3(rodrigues_part_one, rodrigues_part_two),
            look_distance,
        ),
        view_point.position,
    );
}
