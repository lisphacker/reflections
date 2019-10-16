mod geometry;

use geometry::*;

fn hit_sphere(sphere: Sphere, ray: Ray) -> bool {
    let oc = ray.origin - sphere.centre;
    let a = ray.direction * ray.direction;
    let b = 2.0 * (oc * ray.direction);
    let c = (oc * oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(ray: Ray) -> Vector3 {
    if hit_sphere(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5), ray) {
        Vector3::new(1.0, 0.0, 0.0)
    } else {
        let unit_dir = ray.direction.normalize();
        let t = 0.5 * unit_dir.y + 1.0;
        (1.0 - t) * Vector3::one() + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::zero();

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);

            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(ray);

            let ir = (255.0 * col.x) as i32;
            let ig = (255.0 * col.y) as i32;
            let ib = (255.0 * col.z) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
