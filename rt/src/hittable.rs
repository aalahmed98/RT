use crate::vec3::{Point3, Vec3, dot};
use crate::ray::Ray;

pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Point3, normal: Vec3) -> HitRecord{
        HitRecord {p, normal, t, front_face: true}
    }
    pub fn set_face_normal(&mut self, r: &Ray) {
        self.front_face = dot(r.direction, self.normal) < 0.0;
        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}