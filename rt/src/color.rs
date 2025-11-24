use std::ops::{Mul, Add};

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {r, g, b}
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Color{
        Color::new(self*other.r, self*other.g, self*other.b)
    }
}
use std::io::Write;

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let rbyte = (255.999 * pixel_color.r) as usize;
    let gbyte = (255.999 * pixel_color.g) as usize;
    let bbyte = (255.999 * pixel_color.b) as usize;

    writeln!(out, "{rbyte} {gbyte} {bbyte}").unwrap();
}