use std::fmt::Display;

use crate::vec3::{Vec3, Point3};

pub struct Ray {
    orig: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray {orig, dir}
    }

    pub fn at(&self, t: f64 ) -> Point3 {
        self.orig + (t * self.dir)
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

}

impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "orig: {}, dir: {}", self.orig, self.dir)
    }
}