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

use vec3::{Vec3, Point3};
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera; 
use material::{Lambertian, Metal, Dialectric};
use color::Color;
use util::{random_f64, random_f64_range};

fn main() {
    let mut world = HittableList::new();
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));
    for a in -11..11 {
        for b in -11..11 {
            let choose_matt = random_f64();
            let center = Point3::new(a as f64 + 0.9 * random_f64(),
                                    0.2,
                                    b as f64 + 0.9 * random_f64());
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_matt < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphare_material = Lambertian::new(albedo);
                    world.add(Sphere::new(center, 0.2, sphare_material));
                } else if choose_matt < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphare_material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, sphare_material));
                }else {
                    let sphare_material = Dialectric::new(1.5); // Dialectric is glass. fix needed to display properly
                    world.add(Sphere::new(center, 0.2, sphare_material));
                }
            }
        }
    }
    let material1 = Dialectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;     
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}

fn oldmain() {
    //world
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dialectric::new(1.50);
    let material_bubble = Dialectric::new(1.0/1.50);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right));
    // world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(Color::new(0.5, 0.0, 0.0))));
    // world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Color::new(0.0, 0.9, 0.0)))); 

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 20;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}