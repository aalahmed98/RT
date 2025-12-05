use crate::vec3::Point3;
use crate::color::Color;

pub struct Light {
    pub position: Point3,
    pub color: Color,
    pub intensity: f64,
}

impl Light {
    pub fn new(position: Point3, color: Color, intensity: f64) -> Light {
        Light { position, color, intensity }
    }
}

