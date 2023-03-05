pub mod ray;
pub mod vec3;
pub mod shapes;
pub mod camera;
pub mod materials;

use std::sync::Arc;

use rayon::prelude::*;


use crate::materials::{Lambertian, Metal};
use crate::vec3::{Point3, Vec3, Color};
use crate::ray::Ray;
use crate::shapes::{Hit, HittableList, sphere::Sphere};
use crate::camera::Camera;

use image::ImageBuffer;
use piston_window::EventLoop;
use rand::{thread_rng, Rng};


const ASPECT_RATIO: f64 = 16.0/9.0;
const IMAGE_WIDTH: u32 = 400;
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

fn main() {

    
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));

    let material_left   = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right  = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Point3::new(0, 0, -1), 0.5, material_center)));
    world.push(Box::new(Sphere::new(Point3::new(0, -100.5, -1), 100, material_ground)));
    world.push(Box::new(Sphere::new(Point3::new(-1, 0, -1),   0.5, material_left)));
    world.push(Box::new(Sphere::new(Point3::new(1, 0, -1),   0.5, material_right)));

    let cam: Camera = Camera::new();

//    Rendering
    let pixels = (0..IMAGE_HEIGHT)
                .into_par_iter()
                .rev()
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
                }).collect();
           
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