use std::sync::Arc;


use crate::bvh::aabb::Aabb;
use crate::materials::Material;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::hittables::{Hit, HitRecord};


pub struct Sphere {
    center0: Point3, center1: Point3,
    time0: f64, time1: f64,
    radius: f64,
    mat: Arc<dyn Material>,
    is_static: bool,
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    
        let oc: Vec3 = r.origin() - self.center(r.time());

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
        let outward_normal: Vec3 = (p - self.center(r.time())) / self.radius;
        let (u, v) = self.get_sphere_uv(&outward_normal);

        let rec = HitRecord::new(p, root, u, v, r, &outward_normal, self.mat.clone());
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.is_static {
            return Some(Aabb::new(self.center0 - Vec3::new(self.radius, self.radius, self.radius),
            self.center0 + Vec3::new(self.radius, self.radius, self.radius)))    
        }
        let box0 = Aabb::new(self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
                             self.center(time0) + Vec3::new(self.radius, self.radius, self.radius));
        let box1 = Aabb::new(self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
                             self.center(time1) + Vec3::new(self.radius, self.radius, self.radius));                             

        Some(Aabb::surrounding_box(&box0, &box1))
    }
}

impl Sphere {
    pub fn new_static<T: Into<f64>>(cen: Point3, r: T, mat: Arc<dyn Material>) -> Sphere{
        Sphere { 
            center0: cen, 
            center1: cen,
            time0: 0.0, 
            time1: 0.0,
            radius: r.into(),
            mat,
            is_static: true,
        }
    }

    pub fn new_moving<T: Into<f64>>(center0: Point3, 
                                    center1: Point3, 
                                    r: T, 
                                    mat: Arc<dyn Material>, 
                                    time0: f64, 
                                    time1: f64) -> Sphere{
 
        Sphere { 
            center0, center1,
            time0, time1,
            radius: r.into(),
            mat,
            is_static: false
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        if self.is_static {
            return self.center0;
        }
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }

    pub fn get_sphere_uv(&self, p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + std::f64::consts::PI;

        let u = phi / (2.0*std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        (u, v)
    }

}