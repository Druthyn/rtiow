pub mod ray;
pub mod vec3;
pub mod hittables;
pub mod camera;
pub mod materials;
pub mod texture;

use std::sync::Arc;

use hittables::bvh::BVH;
use hittables::constant_medium::ConstantMedium;
use hittables::cube::Cube;
use hittables::transformations::{Translate, RotateY};
use materials::DiffuseLight;
use rayon::prelude::*;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use image::ImageBuffer;
use piston_window::EventLoop;
use rand::{thread_rng, Rng};
use hittables::rectangles::{XyRect, YzRect, XzRect};
use texture::SolidColor;
use texture::checker_texture::CheckerTexture;
use texture::noise_texture::NoiseTexture;
use texture::image_texture::ImageTexture;




use crate::materials::{Lambertian, Metal, Dialectric};
use crate::vec3::{Point3, Vec3, Color};
use crate::ray::Ray;
use crate::hittables::{Hit, HittableList, sphere::Sphere};
use crate::camera::Camera;



const ASPECT_RATIO: f64 = 1.0;
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u64 = 10;
const MAX_DEPTH: i32 = 50;
const TIME0: f64 = 0.0;
const TIME1: f64 = 1.0;

#[allow(dead_code)]
enum DebugSaving {
    Choose,
    Save,
    Quit
}

const SAVE_IMAGE: DebugSaving = DebugSaving::Save;

#[allow(clippy::borrowed_box)]
fn ray_color(r: &Ray, background: &Color, world: &Box<dyn Hit>, depth: i32) -> Color { 

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

#[allow(dead_code)]
fn random_scene() -> Box<dyn Hit> {
    let mut rng = thread_rng();

    let mut world = HittableList::default();


    let checker = Arc::new(Lambertian::new(CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9))));
    world.push(Sphere::new_static(Point3::new(0, -1000, 0), 1000, checker));

    for a in -11..=11 {
        for b in -11..=11 {
            let a_prime= a as f64 + (0.9*rng.gen::<f64>());
            let b_prime= b as f64 + (0.9*rng.gen::<f64>());
            let center = Point3::new(a_prime, 0.2, b_prime);
            
            if (center - Point3::new(4, 0.2, 0)).length() > 0.9 {
                let sphere: Sphere = match rng.gen() {
                    x if (0.0..=0.8).contains(&x) => {
                        let albedo = SolidColor::new(Color::random() * Color::random());
                        let center2 = center + Point3::new(0, rng.gen::<f64>()/2.0, 0);
                        Sphere::new_moving(center, center2, 0.2, Arc::new(Lambertian::new(albedo)), 0.0, 1.0)
                    }
                    x if (0.8..=0.95).contains(&x) => {
                        let albedo = Color::random_in_range(0.5, 1);
                        let fuzz = rng.gen_range(0.0..0.5);
                        Sphere::new_static(center, 0.2, Arc::new(Metal::new(albedo, fuzz)))
                    }
                    _ => {
                        Sphere::new_static(center, 0.2, Arc::new(Dialectric::new(1.5)))                        
                    }
                };
                world.push(sphere);
            }
        }
    }

    let material1 = Arc::new(Dialectric::new(1.5));
    world.push(Sphere::new_static(Point3::new(0, 1, 0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(SolidColor::new_from_rgb(0.4, 0.2, 0.1)));
    world.push(Sphere::new_static(Point3::new(-4, 1, 0), 1.0, material2));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new_static(Point3::new(4, 1, 0), 1.0, material3));

    Box::new(BVH::new(world.list, TIME0, TIME1))
}

#[allow(dead_code)]
fn two_spheres() -> Box<dyn Hit> {
    let mut objects = HittableList::default();

    let checker1 = CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let checker2 = CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    objects.push(Sphere::new_static(Point3::new(0,-10, 0), 10, Arc::new(Lambertian::new(checker1))));
    objects.push(Sphere::new_static(Point3::new(0,10, 0), 10, Arc::new(Lambertian::new(checker2))));

    Box::new(objects)
}

#[allow(dead_code)]
fn two_perlin_spheres() -> Box<dyn Hit> {

    let mut objects = HittableList::default();

    let pertext = NoiseTexture::new(4.0);
    let pertext1 = NoiseTexture::new(4.0);
    

    objects.push(Sphere::new_static(Point3::new(0,-1000, 0), 1000, Arc::new(Lambertian::new(pertext))));
    objects.push(Sphere::new_static(Point3::new(0, 2, 0), 2, Arc::new(Lambertian::new(pertext1))));

    Box::new(objects)    
}

#[allow(dead_code)]
fn earth() -> Box<dyn Hit> {
    let earth_texture = ImageTexture::new("earthmsdap.jpg".to_string());
        
    let earth_surface = Arc::new(Lambertian::new(earth_texture));

    let globe = Sphere::new_static(Point3::new(0,0,0), 2, earth_surface);
    Box::new(globe)
}

#[allow(dead_code)]
fn simple_light() -> Box<dyn Hit> {
    let mut objects = HittableList::default();

    let pertext = NoiseTexture::new(4.0);
    let pertext2 = NoiseTexture::new(4.0);

    let sphere1 = Sphere::new_static(Point3::new(0, -1000, 0), 1000, Arc::new(Lambertian::new(pertext)));
    let sphere2 = Sphere::new_static(Point3::new(0, 2, 0), 2, Arc::new(Lambertian::new(pertext2)));

    objects.push(sphere1);
    objects.push(sphere2);

    let difflight = Arc::new(DiffuseLight::<SolidColor>::new_from_rgb(4,4,4));
    let rect = XyRect::new((3.0,5.0), (1.0,3.0), -2.0, difflight);

    objects.push(rect);
    
    Box::new(BVH::new(vec![Box::new(objects)], 0.0, 1.0))    
}

#[allow(dead_code)]
fn cornell_box() -> Box<dyn Hit> {
    let mut objects = HittableList::default();
    
    let red = Arc::new(Lambertian::new(SolidColor::new_from_rgb(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(SolidColor::new_from_rgb(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(SolidColor::new_from_rgb(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::<SolidColor>::new_from_rgb(15, 15, 15));

    objects.push(YzRect::new((0.0, 555.0), (0.0, 555.0), 555.0, green));
    objects.push(YzRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red));
    objects.push(XzRect::new((213.0, 343.0), (227.0, 332.0), 554.0, light));
    objects.push(XzRect::new((0.0, 555.0), (0.0, 555.0), 0.0, white.clone()));
    objects.push(XzRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()));
    objects.push(XyRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()));

    let cube1 = Cube::new(Point3::zero(), Point3::new(165, 330, 165), white.clone());
    let cube1 = RotateY::new(cube1, 15.0);
    let cube1 = Translate::new(cube1, Vec3::new(265, 0, 295));
    objects.push(cube1);

    let cube2 = Cube::new(Point3::zero(), Point3::new(165, 165, 165), white);
    let cube2 = RotateY::new(cube2, -18.0);
    let cube2 = Translate::new(cube2, Vec3::new(130, 0, 65));
    objects.push(cube2);
    
    Box::new(objects)
}

#[allow(dead_code)]
fn cornell_smoke() -> Box<dyn Hit> {
    let mut objects = HittableList::default();
    
    let red = Arc::new(Lambertian::new(SolidColor::new_from_rgb(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(SolidColor::new_from_rgb(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(SolidColor::new_from_rgb(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::<SolidColor>::new_from_rgb(7, 7, 7));

    objects.push(YzRect::new((0.0, 555.0), (0.0, 555.0), 555.0, green));
    objects.push(YzRect::new((0.0, 555.0), (0.0, 555.0), 0.0, red));
    objects.push(XzRect::new((113.0, 443.0), (127.0, 432.0), 554.0, light));
    objects.push(XzRect::new((0.0, 555.0), (0.0, 555.0), 0.0, white.clone()));
    objects.push(XzRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()));
    objects.push(XyRect::new((0.0, 555.0), (0.0, 555.0), 555.0, white.clone()));

    let cube1 = Cube::new(Point3::zero(), Point3::new(165, 330, 165), white.clone());
    let cube1 = RotateY::new(cube1, 15.0);
    let cube1 = Translate::new(cube1, Vec3::new(265, 0, 295));
    objects.push(ConstantMedium::new_from_color(Box::new(cube1), 0.01, Color::new(0,0,0)));

    let cube2 = Cube::new(Point3::zero(), Point3::new(165, 165, 165), white);
    let cube2 = RotateY::new(cube2, -18.0);
    let cube2 = Translate::new(cube2, Vec3::new(130, 0, 65));
    objects.push(ConstantMedium::new_from_color(Box::new(cube2), 0.01, Color::new(1,1,1)));
    
    Box::new(objects)
}

#[allow(dead_code)]
fn final_scene_book2() -> Box<dyn Hit> {
    let mut boxes1 = HittableList::default();

    let ground = Arc::new(Lambertian::new(SolidColor::new_from_rgb(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;

    let mut rng = thread_rng();

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;

            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;

            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..=100.0);
            let z1 = z0 + w;

            boxes1.push(Cube::new(Point3::new(x0, y0, z0), Point3::new(x1, y1, z1), ground.clone()));
        }
    }

    let mut objects = HittableList::default();
    objects.push(BVH::new(boxes1.list, 0.0, 1.0));

    let light = Arc::new(DiffuseLight::<SolidColor>::new_from_rgb(7, 7, 7));
    objects.push(XzRect::new((123.0, 423.0), (147.0, 412.0), 554.0, light));

    let center1 = Point3::new(400, 400, 200);
    let center2 = center1+ Vec3::new(30, 0, 0);
    let moving_sphere_material = Arc::new(Lambertian::new(SolidColor::new_from_rgb(0.7, 0.3, 0.1)));
    objects.push(Sphere::new_moving(center1, center2, 50, moving_sphere_material, 0.0, 1.0));

    objects.push(Sphere::new_static(Point3::new(260, 150, 45), 50, Arc::new(Dialectric::new(1.5))));
    objects.push(Sphere::new_static(Point3::new(0, 150, 145), 50, Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 1.0))));

    let boundary = Sphere::new_static(Point3::new(360, 150, 145), 70, Arc::new(Dialectric::new(1.5)));
    objects.push(boundary);
    let boundary = Sphere::new_static(Point3::new(360, 150, 145), 70, Arc::new(Dialectric::new(1.5)));
    objects.push(ConstantMedium::new_from_color(Box::new(boundary), 0.2, Color::new(0.2, 0.4, 0.9)));
    let boundary = Sphere::new_static(Point3::new(0, 0, 0), 5000, Arc::new(Dialectric::new(1.5)));
    objects.push(ConstantMedium::new_from_color(Box::new(boundary), 0.0001, Color::new(1, 1, 1)));

    let emat = Arc::new(Lambertian::new(ImageTexture::new("earthmap.jpg".to_string())));
    objects.push(Sphere::new_static(Point3::new(400, 200, 400), 100, emat));
    let pertext = NoiseTexture::new(0.1);
    objects.push(Sphere::new_static(Point3::new(220, 280, 300), 80, Arc::new(Lambertian::new(pertext))));

    let mut boxes2 = HittableList::default();
    let white = Arc::new(Lambertian::new(SolidColor::new_from_rgb(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.push(Sphere::new_static(Point3::random_in_range(0, 165), 10, white.clone()))
    }

    objects.push(Translate::new(
        RotateY::new(
            BVH::new(boxes2.list, 0.0, 1.0),
            15.0
        ),
        Vec3::new(-100, 270, 395)
    ));

    Box::new(objects)
}


fn main() {
    let world: Box<dyn Hit>;
    let cam: Camera;
    let background: Color;

    const SCENE: i32 = 4;
    match SCENE {
        1 => {
            world = random_scene();
            background = Color::new(0.7, 0.8, 1.0);

            let look_from = Point3::new(13, 2, 3);
            let look_at = Point3::new(0, 0, 0);
            cam = Camera::new(
                look_from,
                look_at,
                Vec3::new(0, 1, 0),
                20.0, 
                ASPECT_RATIO,
                0.1,
                10.0,
                TIME0,
                TIME1,
            );
        },
        2 => {
            world = two_spheres();
            background = Color::new(0.7, 0.8, 1.0);

            let look_from = Point3::new(13, 2, 3);
            let look_at = Point3::new(0, 0, 0);
            cam = Camera::new(
                look_from,
                look_at,
                Vec3::new(0, 1, 0),
                20.0, 
                ASPECT_RATIO,
                0.0,
                10.0,
                TIME0,
                TIME1,
            );
        },
        3 => {
            world = two_perlin_spheres();
            background = Color::new(0.7, 0.8, 1.0);

            let look_from = Point3::new(13, 2, 3);
            let look_at = Point3::new(0, 0, 0);
            cam = Camera::new(
                look_from,
                look_at,
                Vec3::new(0, 1, 0),
                20.0, 
                ASPECT_RATIO,
                0.0,
                10.0,
                TIME0,
                TIME1,
            );
        },
        4 => {
            world = earth();
            background = Color::new(0.7, 0.8, 1.0);

            let look_from = Point3::new(13, 2, 3);
            let look_at = Point3::new(0, 0, 0);
            cam = Camera::new(
                look_from,
                look_at,
                Vec3::new(0, 1, 0),
                20.0, 
                ASPECT_RATIO,
                0.0,
                10.0,
                TIME0,
                TIME1,
            );
        },
        5 => {
            world = simple_light();
            background = Color::new(0, 0, 0);
            let look_from = Point3::new(26,3,6);
            let look_at = Point3::new(0,2,0);
            cam = Camera::new(
                look_from,
                look_at,
                Vec3::new(0, 1, 0),
                20.0, 
                ASPECT_RATIO,
                0.0,
                10.0,
                TIME0,
                TIME1,
            )

        },
        6 => {
            world = cornell_box();
            background = Color::new(0, 0, 0);
            let look_from = Point3::new(278, 278, -800);
            let look_at = Point3::new(278, 278, 0);
            cam = Camera::new(
                look_from,
                look_at,
                Vec3::new(0, 1, 0),
                40.0, 
                ASPECT_RATIO,
                0.0,
                10.0,
                TIME0,
                TIME1,
            )
        },
        7 => {
            world = cornell_smoke();
            background = Color::new(0, 0, 0);
            let look_from = Point3::new(278, 278, -800);
            let look_at = Point3::new(278, 278, 0);
            cam = Camera::new(
                look_from,
                look_at,
                Vec3::new(0, 1, 0),
                40.0, 
                ASPECT_RATIO,
                0.0,
                10.0,
                TIME0,
                TIME1,
            )
        },
        8 => {
            world = final_scene_book2();
            background = Color::new(0, 0, 0);
            let look_from = Point3::new(478, 278, -600);
            let look_at = Point3::new(278, 278, 0);
            cam = Camera::new(
                look_from,
                look_at,
                Vec3::new(0, 1, 0),
                40.0, 
                ASPECT_RATIO,
                0.0,
                10.0,
                TIME0,
                TIME1,
            )
        },
        _ => {
            background = Color::new(0.7, 0.8, 1.0);
            world = Box::<HittableList>::default();
            let look_from = Point3::new(0, 0, 0);
            let look_at = Point3::new(0, 1, 0);
            cam = Camera::new(
                look_from,
                look_at,
                Vec3::new(0, 1, 0),
                20.0, 
                ASPECT_RATIO,
                0.0,
                10.0,
                TIME0,
                TIME1,
            );
        }
        ,
    }
    let style = ProgressStyle::with_template("[Elapsed: {elapsed_precise}] Eta: {eta_precise} {bar:40.cyan/blue} {pos:>7}/{len:7}").unwrap();
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
                        pixel_color = pixel_color + ray_color(&r, &background, &world, MAX_DEPTH);
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