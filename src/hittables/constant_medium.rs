use std::{f64::INFINITY, sync::Arc};

use rand::{Rng, thread_rng};

use crate::{materials::{Material, Isotropic}, texture::{Texture, SolidColor}, vec3::{Color, Vec3}, hittables::bvh::aabb::Aabb, ray::Ray};

use super::{Hit, HitRecord};

pub struct ConstantMedium {
    boundary: Box<dyn Hit>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,       
}

impl ConstantMedium {
    pub fn new_from_texture<T: Texture + 'static>(b: Box<dyn Hit>, d: f64, a: T) -> ConstantMedium {
        ConstantMedium { boundary: b, phase_function: Arc::new(Isotropic::new_from_texture(a)), neg_inv_density: -1.0/d }
    }

    pub fn new_from_color(b: Box<dyn Hit>, d: f64, c: Color) -> ConstantMedium {
        ConstantMedium { boundary: b, phase_function: Arc::new(Isotropic::<SolidColor>::new_from_color(c)), neg_inv_density: -1.0/d }
    }
}

impl Hit for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Debugging variables
        let enable_debug = false;
        let mut rng = thread_rng();
        let debugging = enable_debug && rng.gen_bool(0.00001);

        let mut rec1 = self.boundary.hit(r, -INFINITY, INFINITY)?;
        let mut rec2 = self.boundary.hit(r, rec1.t+0.0001, INFINITY)?;

        if debugging {println!("t_min={}, t_max={}", rec1.t, rec2.t)};

        if rec1.t < t_min {rec1.t = t_min}
        if rec2.t > t_max {rec2.t = t_max}

        if rec1.t >= rec2.t {
            return None
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        if debugging {
            println!("hit_distance = {}", hit_distance);
            println!("t = {}", t);
            println!("p = {}", p);
        }

        Some(HitRecord::new(p, t, 0.0, 0.0, r, &Vec3::new(1,0,0), self.phase_function.clone()))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}