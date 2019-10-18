
extern crate rand;

mod vecmath;
mod camera;
mod geometry;

use rand::Rng;
use vecmath::*;
use camera::*;
use geometry::*;

fn color(ray: Ray, world: &HittableList) -> Vector3 {
    match world.hit(ray, 0.0, 1e+35) {
        Some(result) => { 0.5 * (result.normal + 1.0) }
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

fn random() -> f32 {
    rand::thread_rng().gen()
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    
    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::zero();

    let mut world = HittableList::new();
    let s1 = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0);
    world.add(Box::new(s1));
    world.add(Box::new(s2));

    let cam = Camera::new();

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let u = (i as f32 + random()) / (nx as f32);
            let v = (j as f32 + random()) / (ny as f32);
            
            let mut col = Vector3::zero();
            for s in 0..ns {
                let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
                col = col + color(ray, &world);
            }
            col = col / (ns as f32);

            let ir = clamp(255.0 * col.x, 0.0, 255.0) as i32;
            let ig = clamp(255.0 * col.y, 0.0, 255.0) as i32;
            let ib = clamp(255.0 * col.z, 0.0, 255.0) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
