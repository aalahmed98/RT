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
mod light;

use vec3::{Vec3, Point3};
use sphere::Sphere;
use cylinder::Cylinder;
use cube::Cube;
use plane::Plane;
use hittable_list::HittableList;
use camera::Camera; 
use material::{Lambertian, Metal, Dialectric};
use color::Color;
use light::Light;

fn main() {
    let mut world = HittableList::new();
    
    // ============================================================================
    // SCENE 1: SPHERE SCENE
    // ============================================================================
    // Requirements: Scene with a sphere, clear shadows visible
    // Uncomment this section to render Scene 1
    
    // Ground plane - Dark surface to show shadows clearly
    // let ground = Lambertian::new(Color::new(0.2, 0.2, 0.2), 0.0);
    // world.add(Plane::new(
    //     Point3::new(0.0, 0.0, 0.0),
    //     Vec3::new(0.0, 1.0, 0.0),
    //     ground
    // ));
    
    // // Bright sphere - High brightness to cast visible shadows
    // let sphere_mat = Lambertian::new(Color::new(0.8, 0.3, 0.3), 0.6);
    // world.add(Sphere::new(Point3::new(0.0, 1.5, 0.0), 1.0, sphere_mat));
    
    // // Light source positioned to create clear shadows
    // let lights = vec![
    //     Light::new(Point3::new(5.0, 6.0, 3.0), Color::new(1.0, 1.0, 1.0), 1.0),
    // ];
    
    // let mut cam = Camera::new();
    // cam.aspect_ratio = 16.0/9.0;
    // cam.image_width = 600;
    // cam.samples_per_pixel = 500;
    // cam.max_depth = 50;
    // cam.vfov = 20.0;
    // cam.lookfrom = Point3::new(8.0, 3.0, 5.0);
    // cam.lookat = Point3::new(0.0, 1.0, 0.0);
    // cam.vup = Vec3::new(0.0, 1.0, 0.0);
    // cam.defocus_angle = 0.0;
    // cam.focus_dist = 10.0;
    
    // cam.render(&world, &lights);
    
    
    // ============================================================================
    // SCENE 2: PLANE AND CUBE (LOWER BRIGHTNESS)
    // ============================================================================
    // Requirements: Flat plane and cube with lower brightness than Scene 1 sphere
    // Uncomment this section to render Scene 2
    /*
    // Ground plane - Flat surface
    let ground = Lambertian::new(Color::new(0.25, 0.25, 0.25), 0.0);
    world.add(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground
    ));
    
    // Cube with lower brightness than Scene 1 sphere (0.3 < 0.6)
    let cube_mat = Lambertian::new(Color::new(0.3, 0.5, 0.8), 0.3);
    world.add(Cube::from_center_size(
        Point3::new(0.0, 1.0, 0.0),
        1.5,
        cube_mat
    ));
    
    // Light source
    let lights = vec![
        Light::new(Point3::new(4.0, 5.0, 2.0), Color::new(1.0, 1.0, 1.0), 1.0),
    ];
    
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(7.0, 3.0, 4.0);
    cam.lookat = Point3::new(0.0, 1.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    cam.render(&world, &lights);
    
    */
    
    // ============================================================================
    // SCENE 3: ALL OBJECTS (ONE OF EACH)
    // ============================================================================
    // Requirements: One cube, one sphere, one cylinder, one flat plane
    // Clear shadows visible from all objects
    // Uncomment this section to render Scene 3
    /*
    // Flat plane (ground)
    let ground = Lambertian::new(Color::new(0.3, 0.3, 0.3), 0.0);
    world.add(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground
    ));
    
    // One sphere - positioned on the left
    let sphere_mat = Lambertian::new(Color::new(0.8, 0.2, 0.2), 0.5);
    world.add(Sphere::new(Point3::new(-2.5, 1.0, 0.0), 0.8, sphere_mat));
    
    // One cube - positioned in the center
    let cube_mat = Lambertian::new(Color::new(0.2, 0.6, 0.8), 0.4);
    world.add(Cube::from_center_size(
        Point3::new(0.0, 1.0, 0.0),
        1.2,
        cube_mat
    ));
    
    // One cylinder - positioned on the right
    let cylinder_mat = Lambertian::new(Color::new(0.6, 0.8, 0.2), 0.45);
    world.add(Cylinder::new(Point3::new(2.5, 1.0, 0.0), 0.6, 2.0, cylinder_mat));
    
    // Light sources positioned to create clear shadows
    let lights = vec![
        Light::new(Point3::new(5.0, 6.0, 3.0), Color::new(1.0, 1.0, 1.0), 1.2),
        Light::new(Point3::new(-4.0, 4.0, 2.0), Color::new(1.0, 0.95, 0.9), 0.8),
    ];
    
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(8.0, 3.0, 5.0);
    cam.lookat = Point3::new(0.0, 1.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    cam.render(&world, &lights);

    */
    
    // ============================================================================
    // SCENE 4: ALL OBJECTS FROM DIFFERENT PERSPECTIVE
    // ============================================================================
    // Requirements: Same objects as Scene 3, but camera in different position
    // Uncomment this section to render Scene 4
    /*
    // Flat plane (ground) - same as Scene 3
    let ground = Lambertian::new(Color::new(0.3, 0.3, 0.3), 0.0);
    world.add(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground
    ));
    
    // One sphere - Mirror (reflective)
    let sphere_mat = Metal::new(Color::new(0.9, 0.9, 0.9), 0.0, 0.0);  // Perfect mirror
    world.add(Sphere::new(Point3::new(-2.5, 1.0, 0.0), 0.8, sphere_mat));
    
    // One cube - same position as Scene 3
    let cube_mat = Lambertian::new(Color::new(0.2, 0.6, 0.8), 0.4);
    world.add(Cube::from_center_size(
        Point3::new(0.0, 1.0, 0.0),
        1.2,
        cube_mat
    ));
    
    // One cylinder - same position as Scene 3
    let cylinder_mat = Lambertian::new(Color::new(0.6, 0.8, 0.2), 0.45);
    world.add(Cylinder::new(Point3::new(2.5, 1.0, 0.0), 0.6, 2.0, cylinder_mat));
    
    // Same light sources as Scene 3
    let lights = vec![
        Light::new(Point3::new(5.0, 6.0, 3.0), Color::new(1.0, 1.0, 1.0), 1.2),
        //Light::new(Point3::new(-4.0, 4.0, 2.0), Color::new(1.0, 0.95, 0.9), 0.8),
    ];
    
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 35.0;  // Increased field of view to capture all objects
    // Different camera position - viewing from front, moved back and up to see all objects
    cam.lookfrom = Point3::new(0.0, 3.5, 10.0);  // Moved back and slightly lower
    cam.lookat = Point3::new(0.0, 1.0, 0.0);  // Looking at center of scene
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = 12.0;  // Increased focus distance

    cam.render(&world, &lights);
    */
    
    // ============================================================================
    // SCENE 5: TWO CYLINDERS AT DIFFERENT ANGLES
    // ============================================================================
    // Two cylinders with different colors, positioned at different angles
    
    // Ground plane
    let ground = Lambertian::new(Color::new(0.3, 0.3, 0.3), 0.0);
    world.add(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        ground
    ));
    
    // First cylinder - Red/Orange, positioned on the left, taller
    let cylinder1_mat = Lambertian::new(Color::new(0.8, 0.3, 0.2), 0.3);
    world.add(Cylinder::new(Point3::new(-2.0, 1.5, 0.0), 0.5, 3.0, cylinder1_mat));
    
    // Second cylinder - Blue/Purple, positioned on the right, shorter and wider
    let cylinder2_mat = Lambertian::new(Color::new(0.2, 0.4, 0.9), 0.35);
    world.add(Cylinder::new(Point3::new(2.0, 1.0, 0.0), 0.7, 2.0, cylinder2_mat));
    
    // Light source positioned to show shadows clearly
    let lights = vec![
        Light::new(Point3::new(5.0, 6.0, 3.0), Color::new(1.0, 1.0, 1.0), 1.2),
    ];
    
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.vfov = 30.0;
    cam.lookfrom = Point3::new(0.0, 3.0, 8.0);
    cam.lookat = Point3::new(0.0, 1.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.0;

    cam.render(&world, &lights);
    
    
    // ============================================================================
    // NOTE: All scenes are commented out by default
    // Uncomment ONE scene section above to render it
    // Make sure to comment out the default scene if you uncomment a test scene
    // ============================================================================
}
