mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;
mod util;
mod material;

use vec3::{Vec3, Point3, dot};
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera; 
use material::{Lambertian, Metal, Dialectric};
use color::Color;

fn main() {
    //world
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dialectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right));
    // world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(Color::new(0.5, 0.0, 0.0))));
    // world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Color::new(0.0, 0.9, 0.0)))); 

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);
}