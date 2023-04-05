use crate::vec3::{Color, Point3};

use super::{perlin::Perlin, Texture};

#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(sc: f64) -> NoiseTexture {
        NoiseTexture { scale: sc, ..Default::default() }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1,1,1) * self.noise.turb(p * self.scale, None)
        // Color::new(1,1,1) * 0.5 * (1.0 + (self.scale*p.z() + 10.0*self.noise.turb(*p, None)).sin())
    }
}