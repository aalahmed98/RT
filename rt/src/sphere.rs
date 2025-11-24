use crate::vec3::{Point3, dot};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere{center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = dot(r.direction, oc);
        let c = oc.length_squared()  - self.radius*self.radius;
        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None
        }else {
            let sqrtd = discriminant.sqrt();
            let mut root = (h - sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                root = (h + sqrtd) / a;
                if root <= ray_tmin || ray_tmax <= root {
                    return None
                }
            }
            let t = root;
            let p = r.at(t);
            let normal = (p - self.center) / self.radius;
            let mut rec= HitRecord::new(t, p, normal);
            rec.set_face_normal(r);
            Some(rec)
        }
    }
}