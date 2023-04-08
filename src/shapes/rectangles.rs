use std::sync::Arc;
use paste::paste;

use crate::{materials::Material, bvh::aabb::Aabb, ray::Ray, vec3::{Point3, Vec3}};

use super::{Hit, HitRecord};

// todo: macro the duplication out of this section

pub struct XyRect {
    mp: Arc<dyn Material>,
    xs: (f64, f64),
    ys: (f64, f64),
    k: f64,
}

pub struct XzRect {
    mp: Arc<dyn Material>,
    xs: (f64, f64),
    zs: (f64, f64),
    k: f64,
}

pub struct YzRect {
    mp: Arc<dyn Material>,
    ys: (f64, f64),
    zs: (f64, f64),
    k: f64,
}

macro_rules! build_rect_struct {
    ($d1:tt, $d2:tt) => {
        pub struct YzRect {
            mp: Arc<dyn Material>,
            $d1s: (f64, f64),
            zs: (f64, f64),
            k: f64,
        }
    };
}

impl XyRect {
    pub fn new(xs: (f64, f64), ys: (f64, f64), k: f64, mp: Arc<dyn Material>) -> XyRect {
        XyRect {xs, ys, k, mp}
    }
}

impl Hit for XyRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return None
        }

        let x = r.origin().x() + t*r.direction().x();
        let y = r.origin().y() + t*r.direction().y();

        if x < self.xs.0 || x > self.xs.1 || y < self.ys.0 || y > self.ys.1 {
            return None
        }

        let u = (x-self.xs.0)/(self.xs.1-self.xs.0);
        let v = (y-self.ys.0)/(self.ys.1-self.ys.0);
        let outward_normal = &Vec3::new(0,0,1);
        let p = r.at(t);

        Some(HitRecord::new(p, t, u, v, r, outward_normal, self.mp.clone()))
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.xs.0, self.xs.1, self.k-0.0001), 
            Point3::new(self.ys.0, self.ys.1, self.k+0.0001)
        ))
    }
}