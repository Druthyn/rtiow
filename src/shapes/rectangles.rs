use std::sync::Arc;
use paste::paste;

use crate::{materials::Material, bvh::aabb::Aabb, ray::Ray, vec3::{Point3, Vec3}};

use super::{Hit, HitRecord};

macro_rules! build_rect_struct {
    ($d1:tt, $d2:tt plane. Normal in $d3:tt) => {
        paste! {
            pub struct [<$d1:upper $d2 Rect>] {
                mp: Arc<dyn Material>,
                [<$d1 s>]: (f64, f64),
                [<$d2 s>]: (f64, f64),
                k: f64,
            }
        }
        paste! {
            impl [<$d1:upper $d2 Rect>] {
                pub fn new([<$d1 s>]: (f64, f64), [<$d2 s>]: (f64, f64), k: f64, mp: Arc<dyn Material>) -> [<$d1:upper $d2 Rect>] {
                    [<$d1:upper $d2 Rect>] {[<$d1 s>], [<$d2 s>], k, mp}
                }
            }
        }
        paste! {            
            impl Hit for [<$d1:upper $d2 Rect>] {
                fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
                    let t = (self.k - r.origin().$d3()) / r.direction().$d3();
                    if t < t_min || t > t_max {
                        return None
                    }
            
                    let $d1 = r.origin().$d1() + t*r.direction().$d1();
                    let $d2 = r.origin().$d2() + t*r.direction().$d2();
            
                    if $d1 < self.[<$d1 s>].0 || $d1 > self.[<$d1 s>].1 || $d2 < self.[<$d2 s>].0 || $d2 > self.[<$d2 s>].1 {
                        return None
                    }
            
                    let u = ($d1-self.[<$d1 s>].0)/(self.[<$d1 s>].1-self.[<$d1 s>].0);
                    let v = ($d2-self.[<$d2 s>].0)/(self.[<$d2 s>].1-self.[<$d2 s>].0);
                    let outward_normal = &Vec3::new(0,0,1);
                    let p = r.at(t);
            
                    Some(HitRecord::new(p, t, u, v, r, outward_normal, self.mp.clone()))
                }
            
                fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
                    Some(Aabb::new(
                        Point3::new(self.[<$d1 s>].0, self.[<$d1 s>].1, self.k-0.0001), 
                        Point3::new(self.[<$d2 s>].0, self.[<$d2 s>].1, self.k+0.0001)
                    ))
                }
            }
        }
    };
}

build_rect_struct!(x, y plane. Normal in z);
build_rect_struct!(x, z plane. Normal in y);
build_rect_struct!(y, z plane. Normal in x);
