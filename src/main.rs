pub mod ray;
pub mod vec3;
pub mod shapes;
pub mod camera;

use std::thread;
use futures::future::join_all;


use crate::vec3::{Point3, Vec3, Color};
use crate::ray::Ray;
use crate::shapes::{Hittable, HittableList, sphere::Sphere};
use crate::camera::Camera;

use image::{ImageBuffer, Rgba};
use piston_window::EventLoop;
use rand::{thread_rng, Rng};


const ASPECT_RATIO: f64 = 16.0/9.0;
const IMAGE_WIDTH: u32 = 2560;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u64 = 100;

enum DebugSaving {
    Choose,
    Save,
    Quit
}

const SAVE_IMAGE: DebugSaving = DebugSaving::Quit;

async fn fire_ray() -> Color {
    let u = (i as f64 + rng.gen::<f64>()) / ((IMAGE_WIDTH-1)  as f64);
    let v = (j as f64 + rng.gen::<f64>()) / ((IMAGE_HEIGHT-1) as f64);
               
    let r = cam.get_ray(u, v);
    ray_color(&r, &world)
}

fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color { 

    let res = world.hit(r, 0.0, f64::INFINITY);

    if let Some(shape) = res {
        return 0.5 * (shape.get_normal() + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction: Vec3 = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
}



fn main() {

    let mut world = HittableList::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));


    let cam: Camera = Camera::new();

    let mut rng = thread_rng();

//    Rendering

    let mut image_buffer = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            
            let mut pixel_color: Color = Color::zero();


            let mut threads = Vec::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                threads.push(thread::spawn(||->  {
                    async {
                        Color::new(0.0, 0.0, 0.0)
                    }
                }));
            }
        
            join_all(threads);
            let pixel = Rgba::from(pixel_color.to_rgba(255, SAMPLES_PER_PIXEL));
            image_buffer.put_pixel(i, IMAGE_HEIGHT-1-j, pixel);
        }
    }
    println!("\nDone.");

    // Drawing preview window
    
    let mut window: piston_window::PistonWindow =
    piston_window::WindowSettings::new("Scene", [IMAGE_WIDTH, IMAGE_HEIGHT])
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