use crate::hittable::HitRecord;
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Vec3, dot};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
} 

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attentuation = self.albedo;
        Some((attentuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz: if fuzz < 1.0 { fuzz} else {1.0} }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = Vec3::reflect(r_in.direction, rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(rec.p, reflected);
        let attentuation = self.albedo;
        if dot(scattered.direction, rec.normal) > 0.0 {
            Some((attentuation, scattered))
        }else {
            None
        }
        
    }
}

pub struct Dialectric {
    refraction_index: f64
}

impl Dialectric {
    pub fn new(refraction_index: f64) -> Dialectric {
        Dialectric { refraction_index }
    }
}

impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attentuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.direction.unit_vector();
        let refracted = Vec3::refract(unit_direction, rec.normal, ri);
        let scattered = Ray::new(rec.p, refracted);
        Some((attentuation, scattered))
    }
}