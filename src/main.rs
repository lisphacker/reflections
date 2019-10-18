
extern crate rand;
extern crate image;

mod vecmath;
mod camera;
mod geometry;

use std::path::Path;

use rand::Rng;
use image::{Rgb, RgbImage};

use vecmath::*;
use camera::*;
use geometry::*;

fn random() -> f32 {
    rand::thread_rng().gen()
}

fn random_in_unit_sphere() -> Vector3 {
    let mut p = Vector3::one() * 100.0;

    while p * p > 1.0 {
        p = Vector3::new(random(), random(), random()) * 2.0 - 1.0;
    };

    p
}

fn color(ray: Ray, world: &HittableList) -> Vector3 {
    match world.hit(ray, 0.0, 1e+35) {
        Some(result) => { 
            let target = result.point + result.normal + random_in_unit_sphere();
            0.5 * color(Ray::new(result.point, target - result.point), world)
        }
        None => {
            let unit_dir = ray.direction.normalize();
            let t = 0.5 * (unit_dir.y + 1.0);
            (1.0 - t) * Vector3::one() + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    }
    else {
        if x > max {
            max
        }
        else {
            x
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
    let s1 = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0);
    world.add(Box::new(s1));
    world.add(Box::new(s2));

    let cam = Camera::new();
    let mut image = RgbImage::new(nx, ny);

    // println!("P3");
    // println!("{} {}", nx, ny);
    // println!("255");

    for y in 0..ny {
        for x in 0..nx {
            let mut col = Vector3::zero();
            for _s in 0..ns {
                let u = (x as f32 + random()) / (nx as f32);
                let v = (y as f32 + random()) / (ny as f32);
            
                let ray = cam.ray(u, v);
                col = col + color(ray, &world);
            }
            col = col / (ns as f32);

            let ir = clamp(255.0 * col.x.sqrt(), 0.0, 255.0) as u8;
            let ig = clamp(255.0 * col.y.sqrt(), 0.0, 255.0) as u8;
            let ib = clamp(255.0 * col.z.sqrt(), 0.0, 255.0) as u8;

            let p = Rgb([ir, ig, ib]);
            image.put_pixel(x, ny - 1 - y, p);

            // println!("{} {} {}", ir, ig, ib);
        }
    }

    match image.save(Path::new(&output_file_path)) {
        Ok(_) => {}
        Err(_) => { eprintln!("Unable to write to {}", output_file_path); }
    }
}
