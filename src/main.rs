
extern crate rand;
extern crate image;

mod camera;
mod geometry;
mod materials;
mod rays;
mod util;
mod vecmath;

use std::rc::Rc;
use std::path::Path;

use image::{Rgb, RgbImage};

use vecmath::*;
use camera::*;
use geometry::*;
use util::*;
use rays::*;
use materials::*;

const MAX_DEPTH: i32 = 50;

fn color(ray: Ray, world: &HittableList, depth: i32) -> Vector3 {
    match world.hit(ray, 0.0, 1e+35) {
        Some(result) => {
            if depth < MAX_DEPTH {
                match result.material.scatter(ray, &result.geometry) {
                    Some(result2) => { color(result2.scattered_ray, world, depth + 1).scale(result2.attenuation) }
                    None          => { Vector3::zero() }
                }
            }
            else {
                Vector3::zero()
            }
        }
        None => {
            let unit_dir = ray.direction.normalize();
            let t = 0.5 * (unit_dir.y + 1.0);
            (1.0 - t) * Vector3::one() + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let mut args = std::env::args();
    let output_file_path = args.nth(1).expect("Output path could not be opened or written to");

    let nx = 200;
    let ny = 100;
    let ns = 100;
    
    let mut world = HittableList::new();
    let s0 = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Box::from(Lambertian::new(Vector3::new(0.8, 0.3, 0.3)))));
    let s1 = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Box::from(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)))));
    let s2 = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Box::from(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0))));
    let s3 = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Box::from(Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.3))));

    world.add(Box::new(s0));
    world.add(Box::new(s1));
    world.add(Box::new(s2));
    world.add(Box::new(s3));

    let cam = Camera::new();
    let mut image = RgbImage::new(nx, ny);

    for y in 0..ny {
        for x in 0..nx {
            let mut col = Vector3::zero();
            for _s in 0..ns {
                let u = (x as f32 + random()) / (nx as f32);
                let v = (y as f32 + random()) / (ny as f32);
            
                let ray = cam.ray(u, v);
                col = col + color(ray, &world, 0);
            }
            col = col / (ns as f32);

            let ir = clamp(255.0 * col.x.sqrt(), 0.0, 255.0) as u8;
            let ig = clamp(255.0 * col.y.sqrt(), 0.0, 255.0) as u8;
            let ib = clamp(255.0 * col.z.sqrt(), 0.0, 255.0) as u8;

            let p = Rgb([ir, ig, ib]);
            image.put_pixel(x, ny - 1 - y, p);
        }
    }

    match image.save(Path::new(&output_file_path)) {
        Ok(_) => {}
        Err(_) => { eprintln!("Unable to write to {}", output_file_path); }
    }
}
