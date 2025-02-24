use rand::rngs::ThreadRng;
use rand::Rng;

pub fn gen_f64_3(base: f64, range: f64, rng: &mut ThreadRng) -> [f64; 3] {
    return [
        rng.gen_range(base - range..base + range),
        rng.gen_range(base - range..base + range),
        rng.gen_range(base - range..base + range),
    ];
}

pub fn nrmlz_f64_3(a: [f64; 3]) -> [f64; 3] {
    let m = vector_length(a);
    if m > 0.0 {
        return [a[0] / m, a[1] / m, a[2] / m];
    } else {
        return a;
    }
}

pub fn mltply_f64_3(a: [f64; 3], b: f64) -> [f64; 3] {
    return [a[0] * b, a[1] * b, a[2] * b];
}

pub fn vector_length(x: [f64; 3]) -> f64 {
    return (x[0].powi(2) + x[1].powi(2) + x[2].powi(2)).sqrt();
}
