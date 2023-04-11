use image::{ImageBuffer, Rgba};
use indicatif::{ProgressStyle, ParallelProgressIterator};
use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{vec3::Color, hittables::Hit, camera::Camera, ray::Ray};

pub struct ImageSettings {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u64,
    pub max_depth: i32,
}

impl ImageSettings {
    pub fn new(width: u32, aspect_ratio: f64, samples_per_pixel: u64, max_depth: i32) -> Self {
        let height: u32 = (width as f64 / aspect_ratio) as u32;
        ImageSettings { width, height, samples_per_pixel, max_depth }
    }
}

impl Default for ImageSettings {
    fn default() -> Self {
        Self { width: 800, height: 800, samples_per_pixel: 40, max_depth: 50 }
    }
}

pub struct Scene {
    pub background: Color,
    pub world: Box<dyn Hit>,
}

impl Scene {
    pub fn new(bg: Color, world: Box<dyn Hit>) -> Self {
        Scene { background: bg, world }
    }
}

pub struct Renderer {
    pub cam: Camera,
    pub scene: Scene,
    pub image_settings: ImageSettings,
}

impl Renderer {
    pub fn render(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let style = ProgressStyle::with_template("[Elapsed: {elapsed_precise}] Eta: {eta_precise} {bar:40.cyan/blue} {pos:>7}/{len:7}").unwrap();
//    Rendering
    let pixels: Vec<u8> = (0..self.image_settings.height)
                .into_par_iter()
                .progress_with_style(style)
                .flat_map_iter(|j| (0..self.image_settings.width).map(move |i| (i, j)))
                .flat_map_iter(|(i, j)| {
                    let mut pixel_color: Color = Color::zero();
                    let mut rng = thread_rng();

                    for _ in 0..self.image_settings.samples_per_pixel {
                        let u = (i as f64 + rng.gen::<f64>()) / ((self.image_settings.width-1)  as f64);
                        let v = (j as f64 + rng.gen::<f64>()) / ((self.image_settings.height-1) as f64);

                        let r = self.cam.get_ray(u, v);
                        pixel_color = pixel_color + ray_color(&r, &self.scene.background, &self.scene.world, self.image_settings.max_depth);
                    }
                    pixel_color.to_rgba(255, self.image_settings.samples_per_pixel)
                })
                .collect();
    
    let pixels = pixels.chunks(4 * self.image_settings.width as usize) // times 4 due to R G B and A channels for each pixel
                       .rev()                   
                       .flatten()
                       .copied()
                       .collect();
    
    
    ImageBuffer::from_vec(self.image_settings.width, self.image_settings.height, pixels).unwrap()
    }
}

#[allow(clippy::borrowed_box)]
pub fn ray_color(r: &Ray, background: &Color, world: &Box<dyn Hit>, depth: i32) -> Color { 

    if depth <= 0 {
        return Color::zero()
    }

    let res = world.hit(r, 0.0001, f64::INFINITY);

    if let Some(shape) = res {
        let scatter = shape.get_mat().scatter(r, &shape);
        let emitted = shape.get_mat().emitted(shape.u(), shape.v(), &shape.p());
        if let Some((att, scat)) = scatter {
            return emitted + att * ray_color(&scat, background, world, depth-1)
        }
        emitted
    } else {
        *background
    }
}