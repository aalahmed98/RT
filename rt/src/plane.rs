use crate::vec3::{Point3, Vec3};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;

pub struct Plane<'a> {
    pub point: Point3,    // A point on the plane
    pub normal: Vec3,    // Normal vector of the plane
    pub mat: Box<dyn Material + 'a>
}

impl <'a>Plane<'a> {
    pub fn new(point: Point3, normal: Vec3, mat: impl Material + 'a) -> Plane<'a> {
        Plane{ 
            point, 
            normal: normal.unit_vector(),  // Ensure normal is normalized
            mat: Box::new(mat) 
        }
    }
}

impl <'a>Hittable for Plane<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = Vec3::dot(self.normal, r.direction);
        
        // Ray is parallel to plane (no intersection)
        if denom.abs() < 1e-8 {
            return None;
        }
        
        // Calculate t (distance along ray to intersection point)
        let oc = self.point - r.origin;
        let t = Vec3::dot(oc, self.normal) / denom;
        
        // Check if intersection is within ray interval
        if !ray_t.surrounds(t) {
            return None;
        }
        
        let p = r.at(t);
        let mut rec = HitRecord::new(t, p, self.normal, &*self.mat);
        rec.set_face_normal(r);
        Some(rec)
    }
}

