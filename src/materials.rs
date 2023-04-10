use rand::{thread_rng, Rng};

use crate::{ray::Ray, vec3::{Color, Vec3, Point3}, hittables::HitRecord, texture::SolidColor};
use crate::texture::Texture;

pub trait Material :Send + Sync{
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0,0,0)
    }
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

impl Material for Lambertian {            
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal() + Vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal();
        }
        let scattered = Ray::new(rec.p(), scatter_direction, r_in.time());
        let attenuation = self.albedo.value(rec.u(), rec.v(), &rec.p());
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

impl Material for Metal {
            
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(&rec.normal()).unit_vector();

        let scattered = Ray::new(rec.p(), reflected + self.fuzz*Vec3::random_in_unit_sphere(), r_in.time());
        
        if scattered.direction().dot(&rec.normal()) <= 0.0 {
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

impl Material for Dialectric {
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
        let cos_theta = 1.0_f64.min(-unit_direction.dot(&rec.normal()));
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let can_refract = refraction_ratio * sin_theta <= 1.0;
        
        let mut rng = thread_rng();
        let direction = if can_refract && reflectence(cos_theta, refraction_ratio) <= rng.gen(){
            unit_direction.refract(rec.normal(), refraction_ratio)
        } else {
            unit_direction.reflect(&rec.normal())
        };
                            
        let scattered = Ray::new(rec.p(), direction, r_in.time());
        Some((Color::new(1,1,1), scattered))
    }
}

pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

// todo: clean up these constructors, genericise from rgb
impl DiffuseLight {
    pub fn new(a: Box<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emit: a }
    }

    pub fn new_from_color(c: Color) -> DiffuseLight {
        DiffuseLight::new(Box::new(SolidColor::new(c)))
    }

    pub fn new_from_rgb(r: u32, g: u32, b: u32) -> DiffuseLight {
        DiffuseLight::new_from_color(Color::new(r, g, b))
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Box<dyn Texture>,
}

impl Isotropic {
    pub fn new_from_texture(a: Box<dyn Texture>) -> Isotropic {
        Isotropic {albedo: a}
    }

    pub fn new_from_color(c: Color) -> Isotropic {
        Isotropic::new_from_texture(Box::new(SolidColor::new(c)))
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(rec.p(), Vec3::random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u(), rec.v(), &rec.p());
        Some((attenuation, scattered))
    }
}