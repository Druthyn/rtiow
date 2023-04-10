use std::sync::Arc;

use crate::{vec3::Point3, materials::Material, bvh::aabb::Aabb, ray::Ray};

use super::{HittableList, Hit, rectangles::{XyRect, XzRect, YzRect}, HitRecord};

pub struct Cube {
    cube_min: Point3,
    cube_max: Point3,
    sides: HittableList,
}

impl Cube {
    pub fn new(p0: Point3, p1: Point3, mat: Arc<dyn Material>) -> Cube {
        let cube_min = p0;
        let cube_max = p1;

        let mut sides = HittableList::default();

        sides.push(XyRect::new((p0.x(), p1.x()), (p0.y(), p1.y()), p1.z(), mat.clone()));
        sides.push(XyRect::new((p0.x(), p1.x()), (p0.y(), p1.y()), p0.z(), mat.clone()));

        sides.push(XzRect::new((p0.x(), p1.x()), (p0.z(), p1.z()), p1.y(), mat.clone()));
        sides.push(XzRect::new((p0.x(), p1.x()), (p0.z(), p1.z()), p0.y(), mat.clone()));

        sides.push(YzRect::new((p0.y(), p1.y()), (p0.z(), p1.z()), p1.x(), mat.clone()));
        sides.push(YzRect::new((p0.y(), p1.y()), (p0.z(), p1.z()), p0.x(), mat));

        Cube {cube_min, cube_max, sides}
    }
}

impl Hit for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(self.cube_min, self.cube_max))
    }
}