use crate::vec3::Color;
use super::{Texture, SolidColor};

#[derive(Clone)]
pub struct CheckerTexture {
    odd: SolidColor,
    even: SolidColor,
}

impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture { even: SolidColor::new(c1), odd: SolidColor::new(c2) }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &crate::vec3::Point3) -> Color {
        let sines = (10.0*p.x()).sin()*(10.0*p.y()).sin()*(10.0*p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}