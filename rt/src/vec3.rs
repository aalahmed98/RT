#[derive(Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3{
    fn new(x: f64, y: f64, z:f64) -> Vec3{
        Vec3{x, y, z}
    }
    fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

//type alias
pub type Point3 = Vec3;