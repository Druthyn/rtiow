pub mod sphere;
pub mod rectangles;
pub mod cube;


use std::sync::Arc;

use crate::materials::Material;
use crate::vec3::{Point3, Vec3}; 
use crate::ray::Ray;
use crate::bvh::aabb::Aabb;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    mat: Arc<dyn Material>,
    t: f64,
    u: f64,
    v: f64,
    pub front_face: bool
}

impl HitRecord {

    fn new(p: Point3, t: f64, u: f64, v: f64, r: &Ray, outward_normal: &Vec3, mat: Arc<dyn Material>) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {*outward_normal}else{Vec3::zero()-*outward_normal};
        HitRecord {
            p,
            normal,
            mat,
            t,
            u,
            v,
            front_face,
        }
    }

    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    pub fn get_p(&self) -> Point3 {
        self.p
    }

    pub fn get_mat(&self) -> Arc<dyn Material> {
        self.mat.clone()
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> &Vec3 {
        &self.p
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
    }

}


pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}


#[derive(Default)]
pub struct HittableList {
    pub list: Vec<Box<dyn Hit>>
}


impl HittableList {
    pub fn push(&mut self, x: impl Hit + 'static) {
        self.list.push(Box::new(x))
    }
}

impl Hit for HittableList {
    
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        
        let mut closest_res = None;
        let mut closest_so_far = t_max;

        for object in &self.list {
            if let Some(rec) = object.hit(r, t_min, closest_so_far){
                closest_so_far = rec.t;
                closest_res = Some(rec);
            }
        }

        closest_res
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        match self.list.first() {
            Some(first) => 
                match first.bounding_box(time0, time1) {
                    Some(bbox) => 
                        self.list.iter().skip(1).try_fold(bbox, |acc, hitable|
                            hitable.bounding_box(time0, time1).map(|bbox| Aabb::surrounding_box(&acc, &bbox))),
                    _ => None,
                },
            _ => None,
        }
    }
}