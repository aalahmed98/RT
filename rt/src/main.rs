mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod cylinder;
mod cube;
mod plane;
mod interval;
mod camera;
mod util;
mod material;

use vec3::{Vec3, Point3};
use sphere::Sphere;
use cylinder::Cylinder;
use cube::Cube;
use plane::Plane;
use hittable_list::HittableList;
use camera::Camera; 
use material::{Lambertian, Metal, Dialectric};
use color::Color;
use util::{random_f64, random_f64_range};

fn main() {
    let mut world = HittableList::new();
    
    // Ground plane - Reflective (mirror-like)
    let ground_material = Metal::new(Color::new(0.1, 0.2, 0.2), 0.0);  // Slightly blue-tinted mirror
    world.add(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),  // Normal pointing up
        ground_material
    ));
    
    // 1. SPHERE - Matte blue (center)
    let sphere_material = Lambertian::new(Color::new(0.2, 0.4, 0.9));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, sphere_material));
    
    // 2. CYLINDER - Shiny copper (left)
    let cylinder_material = Metal::new(Color::new(0.9, 0.5, 0.3), 0.1);
    world.add(Cylinder::new(Point3::new(-3.0, 1.0, 2.0), 0.6, 2.5, cylinder_material));
    
    // 3. CUBE - Matte emerald green (positioned to be visible in reflections)
    let cube_material = Lambertian::new(Color::new(0.1, 0.7, 0.5));
    world.add(Cube::from_center_size(
        Point3::new(2.5, 1.0, 2.0),  // Moved closer and forward to be in reflection path
        1.5,
        cube_material
    ));
    
    // 4. PLANE - Vertical wall (behind, soft lavender)
    let wall_material = Lambertian::new(Color::new(0.1, 1.0, 0.3));
    world.add(Plane::new(
        Point3::new(0.0, 0.0, -3.0),
        Vec3::new(0.0, 0.0, 1.0),  // Normal pointing forward (toward camera)
        wall_material
    ));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 1000;  // Higher samples = less noise/fuzz (try 1000+ for very clean images)
    cam.max_depth = 50;

    cam.vfov = 20.0;     
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
