use std::{result::{Result}, convert::Into};

use image::{self, DynamicImage, io::Reader, ImageError, GenericImageView};

use crate::vec3::{Color, Vec3};

use super::Texture;

pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: String) -> Result<ImageTexture, ImageError> {
        fn load_image(filename: String) -> Result<DynamicImage, ImageError> {
            Reader::open(filename)?.with_guessed_format()?.decode()
        } 
        let image = match load_image(filename) {
            Ok(x) => Some(x),
            Err(y) => return Err(y)
        };
        let image_data = image.unwrap();
        Ok(ImageTexture {image: image_data})
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
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