use std::f64::INFINITY;

use crate::{vec3::{Vec3, Point3}, ray::Ray};
use crate::hittables::bvh::aabb::Aabb;

use super::{Hit, HitRecord};

pub struct Translate<H : Hit> {
    ptr: H,
    offset: Vec3,
}

impl<H : Hit> Translate<H> {
    pub fn new(ptr: H, offset: Vec3) -> Translate<H> {
        Translate { ptr, offset }
    }
}

impl<H : Hit> Hit for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin()-self.offset, r.direction(), r.time());

        let mut rec = self.ptr.hit(&moved_r, t_min, t_max)?;

        rec.p = rec.p + self.offset;

        rec.set_face_normal(&moved_r, &rec.normal.clone());
        Some(rec)        
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let temp_output_box = self.ptr.bounding_box(time0, time1)?;

        let output_box = Aabb::new(
            temp_output_box.min() + self.offset,
            temp_output_box.max() + self.offset
        );

        Some(output_box)
    }
}

pub struct RotateY<H: Hit> {
    ptr: H,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl<H: Hit> RotateY<H> {
    pub fn new(p: H, angle: f64) -> RotateY<H> {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box(0.0, 1.0).map(|temp_bbox| {
                let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
                let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f64 * temp_bbox.max().x() + (1-i) as f64 * temp_bbox.min().x();
                            let y = j as f64 * temp_bbox.max().y() + (1-j) as f64 * temp_bbox.min().y();
                            let z = k as f64 * temp_bbox.max().z() + (1-k) as f64 * temp_bbox.min().z();

                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;

                            let tester = Vec3::new(newx, y, newz);
                            
                            for c in 0..3 {
                                min[c] = min[c].min(tester[c]);
                                max[c] = max[c].max(tester[c]);
                            }
                        }
                    }    
                }
                Aabb::new(min, max)
            });
        RotateY { ptr: p, sin_theta, cos_theta, bbox }
    }
}

impl<H: Hit> Hit for RotateY<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        macro_rules! rot_hit_helper {
            ($obj:tt, $att:tt) => {
                $att[0] = self.cos_theta*$obj.$att()[0] - self.sin_theta*$obj.$att()[2];
                $att[2] = self.sin_theta*$obj.$att()[0] + self.cos_theta*$obj.$att()[2];
            }
        }

        let mut origin = r.origin(); //todo check if this causes problems? Do I need to copy/clone to avoid fucking my data source
        let mut direction = r.direction();

        rot_hit_helper!(r, origin);
        rot_hit_helper!(r, direction);

        let rotated_r = Ray::new(origin, direction, r.time());

        let rec = self.ptr.hit(&rotated_r, t_min, t_max)?;

        let mut p = rec.p;
        let mut normal = rec.normal;

        rot_hit_helper!(rec, p);
        rot_hit_helper!(rec, normal);
        
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
}