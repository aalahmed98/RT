use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub mat: &'a dyn Material,
}

impl <'a>HitRecord<'a> {
    pub fn new(t: f64, p: Point3, normal: Vec3, mat: &'a dyn Material) -> HitRecord<'a>{
        HitRecord {p, normal, t, front_face: true, mat}
    }
    
    pub fn set_face_normal(&mut self, r: &Ray) {
        self.front_face = Vec3::dot(r.direction, self.normal) < 0.0;
        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}