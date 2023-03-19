use crate::{vec3::Point3, ray::Ray};

pub struct Aabb {
    minimum: Point3,
    maximum: Point3,
}

impl Aabb {
    
    pub fn new(a: Point3, b: Point3) -> Aabb{
        Aabb {
            minimum: a,
            maximum: b,
        }
    }

    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let mut t_min = t_min;
            let mut t_max = t_max;
            let inv_d = 1.0/r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t0 > t_min {
                t_min = t0;
            }
            if t1 < t_max {
                t_max = t1;
            } 
            if t_max <= t_min {
                return false
            }
        }
        true
    }

}