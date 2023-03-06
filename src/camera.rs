use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64
}

impl Camera {

    pub fn new(look_from: Point3, 
               loot_at: Point3, 
               v_up: Vec3, 
               v_fov: f64, 
               aspect_ratio: f64,
               aperture: f64,
               focus_dist: f64) -> Camera {
                
        let theta = v_fov.to_radians();
        let viewport_height = 2.0 * (theta/2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;
                
        let w = (look_from - loot_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = look_from - horizontal/2.0 - vertical/2.0 - focus_dist*w;

        Camera {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
            u, v, w,
            lens_radius: aperture/2.0
        }
    }
   
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(self.origin + offset,
                 self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset
        )
    }
}