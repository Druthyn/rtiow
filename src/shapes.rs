pub mod sphere;


use crate::vec3::{Point3, Vec3}; 
use crate::ray::Ray;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool
}

impl HitRecord {

    fn new(p: Point3, t: f64, r: &Ray, outward_normal: &Vec3) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {*outward_normal}else{Vec3::zero()-*outward_normal};
        HitRecord {
            p,
            normal,
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


}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}




pub struct HittableList<T> {
    objects: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new(object: T) -> Self {
        HittableList {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        
        let mut closest_res = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let result = object.hit(r, t_min, t_max);
            if let Some(rec)  =  result{
                if rec.t < closest_so_far {
                    closest_so_far = rec.t;
                    closest_res = Some(rec);
                }
            }
        }

        closest_res
    }
}