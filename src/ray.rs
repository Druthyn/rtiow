use std::fmt::Display;

use crate::vec3::{Vec3, Point3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, time: f64) -> Ray {
        Ray {orig, dir, time}
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
    
    pub fn time(&self) -> f64 {
        self.time
    }

}

impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "orig: {}, dir: {}", self.orig, self.dir)
    }
}