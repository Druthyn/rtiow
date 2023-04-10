use std::{result::{Result}, convert::Into};

use image::{self, DynamicImage, io::Reader, ImageError, GenericImageView, ImageBuffer, Rgb};

use crate::vec3::{Color, Vec3};

use super::Texture;

#[derive(Clone)]
pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: String) -> ImageTexture {
        fn load_image(filename: String) -> Result<DynamicImage, ImageError> {
            Reader::open(filename)?.with_guessed_format()?.decode()
        } 
        match load_image(filename) {
            Ok(image) => ImageTexture{image},
            Err(_) => ImageTexture::default()
        }
    }

    fn default() -> Self {
        let pixel = Rgb::from([0, 255, 255]);
        let image = ImageBuffer::from_pixel(1, 1, pixel);
        ImageTexture{ image: DynamicImage::ImageRgb8(image)}
    }


}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);
        
        let mut i = (u * Into::<f64>::into(self.image.width())) as u32;
        let mut j = (v * Into::<f64>::into(self.image.height())) as u32;
                
        if i >= self.image.width() {
            i = self.image.width()-1
        }

        if j >= self.image.height() {
            j = self.image.height()-1
        }

        let pixel = self.image.get_pixel(i, j);
        let color_scale = 1.0/255.0;
        Color::new(Into::<f64>::into(pixel.0[0]) * color_scale, 
            Into::<f64>::into(pixel.0[1]) * color_scale, 
            Into::<f64>::into(pixel.0[2]) * color_scale
        )
    }
} 