use crate::vec3::{Color, Point3};

use super::{perlin::Perlin, Texture};

#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(1,1,1) * self.noise.noise(p)
    }
}