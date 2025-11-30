pub fn random_f64() -> f64 {
    fastrand::f64() //random num generator
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}