use crate::hittable::HitRecord;
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Vec3};
use crate::util::random_f64;

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emission(&self) -> Color;
} 

pub struct Lambertian {
    albedo: Color,
    brightness: f64
}

impl Lambertian {
    pub fn new(albedo: Color, brightness: f64) -> Lambertian {
        Lambertian { albedo, brightness }
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
    
    fn emission(&self) -> Color {
        self.brightness * self.albedo
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
    brightness: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64, brightness: f64) -> Metal {
        Metal { albedo, fuzz: if fuzz < 1.0 { fuzz} else {1.0}, brightness }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = Vec3::reflect(r_in.direction, rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(rec.p, reflected);
        let attentuation = self.albedo;
        if Vec3::dot(scattered.direction, rec.normal) > 0.0 {
            Some((attentuation, scattered))
        }else {
            None
        }
        
    }
    
    fn emission(&self) -> Color {
        self.brightness * self.albedo
    }
}

pub struct Dialectric {
    refraction_index: f64,
    brightness: f64
}

impl Dialectric {
    pub fn new(refraction_index: f64, brightness: f64) -> Dialectric {
        Dialectric { refraction_index, brightness }
    }
    pub fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
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
        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction;
        if cannot_refract || self.reflectance(cos_theta, ri) > random_f64() {
            direction = Vec3::reflect(unit_direction, rec.normal);
        } else {
            direction = Vec3::refract(unit_direction, rec.normal, ri);
        }
        let scattered = Ray::new(rec.p, direction);
        Some((attentuation, scattered))
    }
    
    fn emission(&self) -> Color {
        self.brightness * Color::new(1.0, 1.0, 1.0)
    }
}