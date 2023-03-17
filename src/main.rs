pub mod ray;
pub mod vec3;
pub mod shapes;
pub mod camera;
pub mod materials;

use std::sync::Arc;

use materials::Scatter;
use rayon::prelude::*;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use image::ImageBuffer;
use piston_window::EventLoop;
use rand::{thread_rng, Rng};

use crate::materials::{Lambertian, Metal, Dialectric};
use crate::vec3::{Point3, Vec3, Color};
use crate::ray::Ray;
use crate::shapes::{Hit, HittableList, sphere::Sphere};
use crate::camera::Camera;



const ASPECT_RATIO: f64 = 3.0/2.0;
const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u64 = 100;
const MAX_DEPTH: i32 = 50;

enum DebugSaving {
    Choose,
    Save,
    Quit
}

const SAVE_IMAGE: DebugSaving = DebugSaving::Save;

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color { 

    if depth <= 0 {
        return Color::zero()
    }

    let res = world.hit(r, 0.0001, f64::INFINITY);

    if let Some(shape) = res {
        let scatter = shape.get_mat().scatter(r, &shape);
        if let Some((att, scat)) = scatter {
            return att * ray_color(&scat, world, depth-1)
        }
        return Color::new(0,0,0);
    }

    let unit_direction: Vec3 = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1, 1, 1) + t*Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut rng = thread_rng();

    let mut world = HittableList::new();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0, -1000, 0), 1000, ground_material)));

    for a in -11..=11 {
        for b in -11..=11 {
            let a_prime= a as f64 + (0.9*rng.gen::<f64>());
            let b_prime= b as f64 + (0.9*rng.gen::<f64>());
            let center = Point3::new(a_prime, 0.2, b_prime);
            
            if (center - Point3::new(4, 0.2, 0)).length() > 0.9 {
                let sphere_material: Arc<dyn Scatter> = match rng.gen() {
                    x if (0.0..=0.8).contains(&x) => {
                        let albedo = Color::random() * Color::random();
                        Arc::new(Lambertian::new(albedo))
                    }
                    x if (0.8..=0.95).contains(&x) => {
                        let albedo = Color::random_in_range(0.5, 1);
                        let fuzz = rng.gen_range(0.0..0.5);
                        Arc::new(Metal::new(albedo, fuzz))
                    }
                    _ => {
                        Arc::new(Dialectric::new(1.5))
                    }
                };
                world.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dialectric::new(1.5));
    world.push(Box::new(Sphere::new(Point3::new(0, 1, 0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(Point3::new(-4, 1, 0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(Point3::new(4, 1, 0), 1.0, material3)));

    world
}

fn main() {
    
    let world = random_scene();

    let look_from = Point3::new(13, 2, 3);
    let look_at = Point3::new(0, 0, 0);
    let cam: Camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0, 1, 0),
        20.0, 
        ASPECT_RATIO,
        0.1,
        10.0
    );

    let style = ProgressStyle::with_template("[{elapsed} elapsed] [Expected in: {eta}] {wide_bar:40.cyan/blue} {pos:>  7}/{len:7}").unwrap();
//    Rendering
    let pixels: Vec<u8> = (0..IMAGE_HEIGHT)
                .into_par_iter()
                .progress_with_style(style)
                .flat_map_iter(|j| (0..IMAGE_WIDTH).map(move |i| (i, j)))
                .flat_map_iter(|(i, j)| {
                    let mut pixel_color: Color = Color::zero();
                    let mut rng = thread_rng();

                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = (i as f64 + rng.gen::<f64>()) / ((IMAGE_WIDTH-1)  as f64);
                        let v = (j as f64 + rng.gen::<f64>()) / ((IMAGE_HEIGHT-1) as f64);

                        let r = cam.get_ray(u, v);
                        pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
                    }
                    pixel_color.to_rgba(255, SAMPLES_PER_PIXEL)
                })
                .collect();
    
    let pixels = pixels.chunks(4 * IMAGE_WIDTH as usize) // times 4 due to R G B and A channels for each pixel
                       .rev()                   
                       .flatten()
                       .copied()
                       .collect();
    
    let image_buffer = ImageBuffer::from_vec(IMAGE_WIDTH, IMAGE_HEIGHT, pixels).unwrap();
    
    println!("\nDone.");

    // Drawing preview window
    
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::new("Scene", [IMAGE_WIDTH, IMAGE_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|_e| { panic!("Could not create window!")});

    let texture = piston_window::Texture::from_image(
        &mut window.create_texture_context(),
        &image_buffer,
        &piston_window::TextureSettings::new())
        .unwrap();

    window.set_lazy(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            piston_window::clear([1.0; 4], g);
            piston_window::image(&texture, c.transform, g)
        });
    }


    // Save or discard image
    
    match SAVE_IMAGE {
        DebugSaving::Save => image_buffer.save("image.png").unwrap(),
        DebugSaving::Quit => (),
        DebugSaving::Choose => {
        
            let mut input = String::new();
            
            let mut valid = false;
            
            while !valid {

                println!("Save image (s) or quit (q)?");
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let san_input = input.trim();
                
                
                if ["s", "S"].contains(&san_input) {
                    image_buffer.save("image.png").unwrap();
                    println!("image.png saved to working directory");
                    valid = true;
                } else if ["q", "Q"].contains(&san_input) {
                    valid = true;
                } else {
                    println!("Invalid input.");
                }
            }
        }
    }
}