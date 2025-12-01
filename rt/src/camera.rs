use crate::hittable::Hittable;
use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::vec3::{Vec3, Point3};
use crate::util::random_f64;
use std::io::Write;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,

    //private
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera{
    pub fn new() -> Camera{ 
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            ..Default::default()
        }
    }
    
    pub fn render(&mut self, world: &(impl Hittable + Sync)) {
        self.initialize();
        let mut out = std::io::stdout();

        writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_height).unwrap();

        // Create a vector of all pixel coordinates
        let total_pixels = self.image_width * self.image_height;
        let pixels: Vec<(usize, usize)> = (0..self.image_height)
            .flat_map(|j| (0..self.image_width).map(move |i| (i, j)))
            .collect();

        // Create progress bar
        let pb = ProgressBar::new(total_pixels as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} pixels ({percent}%) {msg}")
                .unwrap()
                .progress_chars("#>-")
        );
        pb.set_message("Rendering");

        // Process pixels in parallel with progress tracking
        let pixel_colors: Vec<Color> = pixels
            .par_iter()
            .map(|&(i, j)| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                pb.inc(1);
                self.pixel_samples_scale * pixel_color
            })
            .collect();

        pb.finish_with_message("Done!");

        // Write pixels in order
        for pixel_color in pixel_colors {
            write_color(&mut out, pixel_color);
        }
    }
     
    fn initialize(&mut self){
         //calculate the image hight
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.image_height = if self.image_height < 1 {1} else {self.image_height};

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;
        //let focal_length = (self.lookfrom - self.lookat).length();
        let theta = self.vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;

       // let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = (Vec3::cross(self.vup, self.w)).unit_vector();
        self.v = Vec3::cross(self.w,self.u);


        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - (self.focus_dist * self.w)
                - viewport_u / 2.0 - viewport_v / 2.0;
               
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * ((self.defocus_angle / 2.0).to_radians()).tan();
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc 
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;
        let ray_origin = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    
    fn ray_color(&self, r: &Ray, depth: usize, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0)
        }
        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {

            if let Some((attentuation, scattered)) = rec.mat.scatter(r, &rec) {
                return attentuation * self.ray_color(&scattered, depth-1, world);
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }
        let unit_direction = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
    }

}
