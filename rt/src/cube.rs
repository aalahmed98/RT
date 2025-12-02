use crate::vec3::{Point3, Vec3};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;

pub struct Cube<'a> {
    pub min: Point3,  // Minimum corner (bottom-left-back)
    pub max: Point3,   // Maximum corner (top-right-front)
    pub mat: Box<dyn Material + 'a>
}

impl <'a>Cube<'a> {
    pub fn new(min: Point3, max: Point3, mat: impl Material + 'a) -> Cube<'a> {
        Cube{ min, max, mat: Box::new(mat) }
    }
    
    // Helper function to create a cube from center and size
    pub fn from_center_size(center: Point3, size: f64, mat: impl Material + 'a) -> Cube<'a> {
        let half = size / 2.0;
        Cube::new(
            Point3::new(center.x - half, center.y - half, center.z - half),
            Point3::new(center.x + half, center.y + half, center.z + half),
            mat
        )
    }
}

impl <'a>Hittable for Cube<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;
        
        // Check intersection with each pair of parallel planes (slab method)
        for axis in 0..3 {
            let inv_d = 1.0 / match axis {
                0 => r.direction.x,
                1 => r.direction.y,
                2 => r.direction.z,
                _ => unreachable!(),
            };
            
            let origin = match axis {
                0 => r.origin.x,
                1 => r.origin.y,
                2 => r.origin.z,
                _ => unreachable!(),
            };
            
            let min_bound = match axis {
                0 => self.min.x,
                1 => self.min.y,
                2 => self.min.z,
                _ => unreachable!(),
            };
            
            let max_bound = match axis {
                0 => self.max.x,
                1 => self.max.y,
                2 => self.max.z,
                _ => unreachable!(),
            };
            
            let mut t0 = (min_bound - origin) * inv_d;
            let mut t1 = (max_bound - origin) * inv_d;
            
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            
            if t_max <= t_min {
                return None;
            }
        }
        
        // Check if intersection is within ray interval
        if !ray_t.surrounds(t_min) {
            return None;
        }
        
        let t = t_min;
        let p = r.at(t);
        
        // Calculate normal based on which face was hit
        let mut normal = Vec3::zero();
        let epsilon = 1e-6;
        
        // Determine which face was hit by checking which plane is closest
        let dist_to_min_x = (p.x - self.min.x).abs();
        let dist_to_max_x = (p.x - self.max.x).abs();
        let dist_to_min_y = (p.y - self.min.y).abs();
        let dist_to_max_y = (p.y - self.max.y).abs();
        let dist_to_min_z = (p.z - self.min.z).abs();
        let dist_to_max_z = (p.z - self.max.z).abs();
        
        let min_dist = dist_to_min_x.min(dist_to_max_x)
            .min(dist_to_min_y.min(dist_to_max_y))
            .min(dist_to_min_z.min(dist_to_max_z));
        
        if (dist_to_min_x - min_dist).abs() < epsilon {
            normal = Vec3::new(-1.0, 0.0, 0.0);
        } else if (dist_to_max_x - min_dist).abs() < epsilon {
            normal = Vec3::new(1.0, 0.0, 0.0);
        } else if (dist_to_min_y - min_dist).abs() < epsilon {
            normal = Vec3::new(0.0, -1.0, 0.0);
        } else if (dist_to_max_y - min_dist).abs() < epsilon {
            normal = Vec3::new(0.0, 1.0, 0.0);
        } else if (dist_to_min_z - min_dist).abs() < epsilon {
            normal = Vec3::new(0.0, 0.0, -1.0);
        } else if (dist_to_max_z - min_dist).abs() < epsilon {
            normal = Vec3::new(0.0, 0.0, 1.0);
        }
        
        let mut rec = HitRecord::new(t, p, normal, &*self.mat);
        rec.set_face_normal(r);
        Some(rec)
    }
}

