use std::ops::{Mul, Add, AddAssign};
use crate::vec3::{Vec3};
use crate::interval::{Interval};

#[derive(Clone,Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }
}
impl From<Vec3> for Color {
    fn from(v: Vec3) -> Color {
        Color::new(v.x, v.y, v.z)
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}
impl Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color{
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }

}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Color{
        Color::new(self*other.r, self*other.g, self*other.b)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other : Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}
use std::io::Write;

const intensity: Interval = Interval::new(0.0, 0.999);

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    }else {
        0.0
    }
}
pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let r = linear_to_gamma(pixel_color.r);
    let g = linear_to_gamma(pixel_color.g);
    let b = linear_to_gamma(pixel_color.b);
    let rbyte = (256.0 * intensity.clamp(r)) as usize;
    let gbyte = (256.0 * intensity.clamp(g)) as usize;
    let bbyte = (256.0 * intensity.clamp(b)) as usize;

    writeln!(out, "{rbyte} {gbyte} {bbyte}").unwrap();
}