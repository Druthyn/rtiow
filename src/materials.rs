use crate::{ray::Ray, vec3::{Color, Vec3}, shapes::HitRecord};

pub trait Scatter :Send + Sync{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Lambertian {
            albedo: a,
        }
    }
}

impl Scatter for Lambertian {            
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.get_normal() + Vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.get_normal();
        }
        let scattered = Ray::new(rec.get_p(), scatter_direction);
        Some((self.albedo, scattered))
    }
}


pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, fuzz: f64) -> Self {
        Metal {
            albedo: a,
            fuzz,
        }
    }
}

impl Scatter for Metal {
            
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(&rec.get_normal()).unit_vector();

        let scattered = Ray::new(rec.get_p(), reflected + self.fuzz*Vec3::random_in_unit_sphere());
        
        if scattered.direction().dot(&rec.get_normal()) <= 0.0 {
            return None
        }

        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}