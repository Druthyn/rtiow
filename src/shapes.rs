pub mod sphere;


use std::sync::Arc;

use crate::materials::Scatter;
use crate::vec3::{Point3, Vec3}; 
use crate::ray::Ray;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    mat: Arc<dyn Scatter>,
    t: f64,
    front_face: bool
}

impl HitRecord {

    fn new(p: Point3, t: f64, r: &Ray, outward_normal: &Vec3, mat: Arc<dyn Scatter>) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {*outward_normal}else{Vec3::zero()-*outward_normal};
        HitRecord {
            p,
            normal,
            mat,
            t,
            front_face,
        }
    }

    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    pub fn get_p(&self) -> Point3 {
        self.p
    }

    pub fn get_mat(&self) -> Arc<dyn Scatter> {
        self.mat.clone()
    }


}


pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type HittableList = Vec<Box<dyn Hit>>;

impl Hit for HittableList {
    
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        
        let mut closest_res = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far){
                closest_so_far = rec.t;
                closest_res = Some(rec);
            }
        }

        closest_res
    }
}