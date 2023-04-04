pub mod checker_texture;

use crate::vec3::{Point3, Color};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> SolidColor {
        SolidColor { color_value: c }
    }

    pub fn new_from_rgb(red: f64, green: f64, blue: f64) -> SolidColor{
        SolidColor { color_value: Color::new(red, green, blue)}
    }

}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}