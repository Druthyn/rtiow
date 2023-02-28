use crate::ASPECT_RATIO;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {

    pub fn new() -> Camera{
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        let origin: Point3 = Vec3::zero();
        let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0, 0);
        let vertical: Vec3 = Vec3::new(0, VIEWPORT_HEIGHT, 0);
        let lower_left_corner: Point3 = origin - horizontal/2.0 - (vertical/2.0) - Vec3::new(0, 0, FOCAL_LENGTH);  

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical            
        }
    }
   
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}




