use rand::rngs::ThreadRng;
use rand::Rng;
use std::f32::consts::PI;

pub fn sbtr_f32_3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

pub fn dd_f32_3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

pub fn mltply_f32_3(a: [f32; 3], b: f32) -> [f32; 3] {
    return [a[0] * b, a[1] * b, a[2] * b];
}

pub fn nrmlz_f32_3(a: [f32; 3]) -> [f32; 3] {
    let m = vector_length(a);
    if m > 0.0 {
        return [a[0] / m, a[1] / m, a[2] / m];
    } else {
        return a;
    }
}

pub fn dstnc_f32_3(a: [f32; 3], b: [f32; 3]) -> f32 {
    return vector_length(sbtr_f32_3(b, a));
}

pub fn average_f32_3(a: Vec<[f32; 3]>) -> [f32; 3] {
    let mut b: [f32; 3] = [0.0, 0.0, 0.0];
    for i in 0..a.len() {
        b = dd_f32_3(b, a[i]);
    }

    return mltply_f32_3(b, 1.0 / (a.len() as f32));
}

pub fn vector_length(x: [f32; 3]) -> f32 {
    return (x[0].powi(2) + x[1].powi(2) + x[2].powi(2)).sqrt();
}

pub fn gen_f32_3(base: f32, range: f32, rng: &mut ThreadRng) -> [f32; 3] {
    return [
        rng.gen_range(base - range..base + range),
        rng.gen_range(base - range..base + range),
        rng.gen_range(base - range..base + range),
    ];
}

pub fn gen_rthgnl_f32_3(a: [f32; 3], rng: &mut ThreadRng) -> [f32; 3] {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    if a[2] != 0.0 {
        x = rng.gen_range(-1.0..1.0);
        y = rng.gen_range(-1.0..1.0);
        z = ((-1.0 * a[0] * x) - (a[1] * y)) / a[2];
    } else if a[1] != 0.0 {
        x = rng.gen_range(-1.0..1.0);
        z = rng.gen_range(-1.0..1.0);
        y = ((-1.0 * a[0] * x) - (a[2] * z)) / a[1];
    } else if a[0] != 0.0 {
        y = rng.gen_range(-1.0..1.0);
        z = rng.gen_range(-1.0..1.0);
        x = ((-1.0 * a[1] * y) - (a[2] * z)) / a[0];
    } else {
        // random vector as default for 0 vector received
        while vector_length([x, y, z]) == 0.0 {
            x = rng.gen_range(-1.0..1.0);
            y = rng.gen_range(-1.0..1.0);
            z = rng.gen_range(-1.0..1.0);
        }
    }
    return nrmlz_f32_3([x, y, z]);
}

pub fn find_orthogonal_f32_3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    return nrmlz_f32_3([
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]);
}

// pub fn find_longitudinal_plane_normal(c: [f32; 3], x: [f32; 3], y: [f32; 3]) -> ([f32; 3]) {
//     return nrmlz_f32_3(dd_f32_3(sbtr_f32_3(x, c), sbtr_f32_3(y, c)));
// }

pub fn find_points_normal(x: [f32; 3], y: [f32; 3]) -> [f32; 3] {
    return nrmlz_f32_3(sbtr_f32_3(y, x));
}

pub fn dot_product(a: [f32; 3], b: [f32; 3]) -> f32 {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

pub fn angle_360_of(c: [f32; 3], x: [f32; 3], r: [f32; 3], norm: [f32; 3]) -> f32 {
    let diff = sbtr_f32_3(c, x);
    if vector_length(diff) == 0.0 {
        return 0.0;
    }

    let vector = nrmlz_f32_3(diff);
    let mut angle_of = dot_product(vector, r) / (vector_length(vector) * vector_length(r));

    if angle_of > 1.0 {
        angle_of = 1.0;
    }

    if angle_of < -1.0 {
        angle_of = -1.0;
    }

    let angle = angle_of.acos();

    let n = find_orthogonal_f32_3(norm, r);

    // (v−p)⋅n>0
    if dot_product(diff, n) < 0.0 {
        return 2.0 * PI - angle;
    }

    return angle;
}

pub fn angular_difference(a: f32, b: f32) -> f32 {
    let diffbst = (a - b).abs();
    if diffbst <= PI {
        return diffbst;
    } else {
        return 2.0 * PI - diffbst;
    }
}

pub fn gen_f32_3_unit_on_point_normal_plane(
    planes_normal: [f32; 3],
    planes_point: [f32; 3],
    unit: f32,
    rng: &mut ThreadRng,
) -> [f32; 3] {
    let random_vector_on_plane = gen_rthgnl_f32_3(planes_normal, rng);
    return dd_f32_3(mltply_f32_3(random_vector_on_plane, unit), planes_point);
}
