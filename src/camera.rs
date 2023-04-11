use rand::{thread_rng, Rng};

use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[allow(dead_code)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

pub struct CameraSettings {
    v_fov: f64, 
    pub aspect_ratio: f64,
    apeture: f64,
    focus_dist: f64,
}

impl CameraSettings {
    pub fn new<T1: Into::<f64>, T2: Into::<f64>, T3: Into::<f64>, T4: Into::<f64>,>(v_fov: T1, aspect_ratio: T2, apeture: T3, focus_dist: T4) -> CameraSettings {
        CameraSettings {
            v_fov: v_fov.into(), 
            aspect_ratio: aspect_ratio.into(), 
            apeture: apeture.into(), 
            focus_dist: focus_dist.into()
        }
    }
}

impl Default for CameraSettings {
    fn default() -> Self {
        let v_fov = 40.0;
        let aspect_ratio = 16.0/9.0;
        let apeture = 0.0;
        let focus_dist = 10.0;      
        Self { v_fov, aspect_ratio, apeture, focus_dist }
    }
}


impl Camera {
    pub fn new<T1: Into::<f64>, T2: Into::<f64>>(look_from: Point3, 
               look_at: Point3, 
               v_up: Vec3,
               settings: CameraSettings,           
               time0: T1,
               time1: T2) -> Camera {
                
        let theta = settings.v_fov.to_radians();
        let viewport_height = 2.0 * (theta/2.0).tan();
        let viewport_width = settings.aspect_ratio * viewport_height;
                
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let horizontal = settings.focus_dist * viewport_width * u;
        let vertical = settings.focus_dist * viewport_height * v;
        let lower_left_corner = look_from - horizontal/2.0 - vertical/2.0 - settings.focus_dist*w;

        Camera {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
            u, v, w,
            lens_radius: settings.apeture/2.0,
            time0: time0.into(), time1: time1.into()
        }
    }
   
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        let mut rng = thread_rng();

        Ray::new(self.origin + offset,
                 self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset,
                 rng.gen_range(self.time0..self.time1)
        )
    }
}

impl Default for Camera {
    fn default() -> Camera {
        let settings = CameraSettings::default();
        let theta = settings.v_fov.to_radians();
        let look_from = Point3::new(0,0,0);
        let look_at = Point3::new(0,0,1);
        let v_up = Vec3::new(0,1,0);
        

        let viewport_height = 2.0 * (theta/2.0).tan();
        let viewport_width = settings.aspect_ratio * viewport_height;
                
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let horizontal = settings.focus_dist * viewport_width * u;
        let vertical = settings.focus_dist * viewport_height * v;
        let lower_left_corner = look_from - horizontal/2.0 - vertical/2.0 - settings.focus_dist*w;
        Camera { origin: look_from, lower_left_corner, horizontal, vertical, u, v, w, lens_radius: settings.apeture/2.0, time0: 0.0, time1: 1.0 }
    }
}