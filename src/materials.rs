use rand::{thread_rng, Rng};

use crate::{ray::Ray, vec3::{Color, Vec3}, shapes::HitRecord};
use crate::texture::Texture;

pub trait Scatter :Send + Sync{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: Box<dyn Texture>) -> Self {
        Lambertian {
            albedo: a,
        }
    }
}

impl Scatter for Lambertian {            
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.get_normal() + Vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.get_normal();
        }
        let scattered = Ray::new(rec.get_p(), scatter_direction, r_in.time());
        let attenuation = self.albedo.value(rec.u(), rec.v(), rec.p());
        Some((attenuation, scattered))
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

        let scattered = Ray::new(rec.get_p(), reflected + self.fuzz*Vec3::random_in_unit_sphere(), r_in.time());
        
        if scattered.direction().dot(&rec.get_normal()) <= 0.0 {
            return None
        }

        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Dialectric {
    ir: f64
}

impl Dialectric {
    pub fn new(index_of_refraction: f64) -> Self{
        Dialectric {
            ir: index_of_refraction
        }
    }
}

impl Scatter for Dialectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        fn reflectence(cosine: f64, ref_idx: f64) -> f64 {
            let r0 = ((1.0-ref_idx) / (1.0+ref_idx)).powi(2);
            r0 + (1.0-r0)*(1.0-cosine).powi(5)
    
        }

        let refraction_ratio = match rec.front_face {
            true => 1.0/self.ir,
            false => self.ir,
        };

        let unit_direction = r_in.direction() .unit_vector();
        let cos_theta = 1.0_f64.min(-unit_direction.dot(&rec.get_normal()));
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let can_refract = refraction_ratio * sin_theta <= 1.0;
        
        let mut rng = thread_rng();
        let direction = if can_refract && reflectence(cos_theta, refraction_ratio) <= rng.gen(){
            unit_direction.refract(rec.get_normal(), refraction_ratio)
        } else {
            unit_direction.reflect(&rec.get_normal())
        };
                            
        let scattered = Ray::new(rec.get_p(), direction, r_in.time());
        Some((Color::new(1,1,1), scattered))
    }
}