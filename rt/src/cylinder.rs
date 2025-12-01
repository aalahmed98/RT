use crate::vec3::{Point3, Vec3};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;

pub struct Cylinder<'a> {
    pub center: Point3,      // Center of the cylinder
    pub radius: f64,
    pub height: f64,
    pub mat: Box<dyn Material + 'a>
}

impl <'a>Cylinder<'a> {
    pub fn new(center: Point3, radius: f64, height: f64, mat: impl Material + 'a) -> Cylinder<'a> {
        Cylinder{ center, radius, height, mat: Box::new(mat) }
    }
}

impl <'a>Hittable for Cylinder<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let y_min = self.center.y - self.height / 2.0;
        let y_max = self.center.y + self.height / 2.0;
        
        let oc = Vec3::new(
            self.center.x - r.origin.x,
            0.0,  // y component not used in quadratic
            self.center.z - r.origin.z
        );
        let a = r.direction.x * r.direction.x + r.direction.z * r.direction.z;
        if a < 1e-8 {  // Ray is parallel to cylinder axis
            return None;
        }
        let h = r.direction.x * oc.x + r.direction.z * oc.z;
        let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;
        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let p = r.at(root);
        if p.y < y_min || p.y > y_max {
            // Try the other root
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
            let p2 = r.at(root);
            if p2.y < y_min || p2.y > y_max {
                return None;
            }
            // Use second root
            let t = root;
            let p = p2;
            let normal = Vec3::new(
                (p.x - self.center.x) / self.radius,
                0.0,
                (p.z - self.center.z) / self.radius
            );
            let mut rec = HitRecord::new(t, p, normal, &*self.mat);
            rec.set_face_normal(r);
            Some(rec)
        } else {
            // First root is valid
            let t = root;
            let normal = Vec3::new(
                (p.x - self.center.x) / self.radius,
                0.0,
                (p.z - self.center.z) / self.radius
            );
            let mut rec = HitRecord::new(t, p, normal, &*self.mat);
            rec.set_face_normal(r);
            Some(rec)
        }
    }
}