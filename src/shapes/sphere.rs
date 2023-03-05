use std::sync::Arc;

use crate::materials::Scatter;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::shapes::{Hit, HitRecord};


pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Scatter>
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    
        let oc: Vec3 = r.origin() - self.center;

        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {return None};
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b -sqrtd)/a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd)/a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal: Vec3 = (p - self.center) / self.radius;

        let rec = HitRecord::new(p, root, r, &outward_normal, self.mat.clone());
        Some(rec)
    }
}

impl Sphere {
    pub fn new<T: Into<f64>>(cen: Point3, r: T, mat: Arc<dyn Scatter>) -> Sphere{
        Sphere { 
            center: cen, 
            radius: r.into(),
            mat,
        }
    }
}