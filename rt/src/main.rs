mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;

use color::*;
use vec3::{Vec3, Point3, dot};
use ray::{Ray};
use sphere::Sphere;
use hittable::Hittable;
use hittable_list::{HittableList};

// fn hit_sphare(center: Point3, radius: f64, r: &Ray) -> f64 {
//     let oc = center - r.origin;
//     let a = r.direction.length_squared();
//     let h = dot(r.direction, oc);
//     let c = oc.length_squared()  - radius*radius;
//     let discriminant = h*h - a*c;
//     if discriminant < 0.0 {
//         -1.0
//     }else {
//         (h - discriminant.sqrt()) / a
//     }
// }

fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (Color::new(rec.normal.x, rec.normal.y, rec.normal.z) + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 /9.0;
    let image_width = 400;

    //calculate the image hight
    let mut image_height = (image_width as f64 / aspect_ratio) as usize;
    image_height = if image_height < 1 {1} else {image_height};

    //world
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)); 

    //Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::zero();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center 
        - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v / 2.0;
    let pixal00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut out = std::io::stdout();

    println!("P3\n {image_width} {image_height} \n255\n");
    for j in 0..image_height {
        eprint!("\nScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixal00_loc + (i as f64 * pixel_delta_u)
                + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r, &world);
           write_color(&mut out, pixel_color);
        }
    }
    
    eprint!("\n->Image generation is completed\n");
}