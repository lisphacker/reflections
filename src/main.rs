use std::ops;

#[derive(Debug, Copy, Clone)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl ops::Add for Vector3 {
    type Output = Self;

    fn add(self: Self, operand: Self) -> Self {
        Vector3::new(self.x + operand.x, self.y + operand.y, self.z + operand.z)
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self: Self, multiplier: f32) -> Self {
        Vector3::new(self.x * multiplier, self.y * multiplier, self.z * multiplier)
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self: Self, multiplier: Vector3) -> Vector3 {
        Vector3::new(self * multiplier.x, self * multiplier.y, self * multiplier.z)
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Self;

    fn div(self: Self, divisor: f32) -> Self {
        self * (1.0 / divisor)
    }
}

impl Vector3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x: x, y: y, z: z}
    }

    fn zero() -> Self {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    fn one() -> Self {
        Vector3 { x: 1.0, y: 1.0, z: 1.0 }
    }

    fn copy(v: &Self) -> Self {
        return Vector3 {
            x: v.x,
            y: v.y,
            z: v.z
        }
    }

    fn length(self: Self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }

    fn normalize(self: Self) -> Self {
        self / self.length()
    }
}

struct Ray {
    origin:    Vector3,
    direction: Vector3
}

impl Ray {
    fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray { 
            origin:    origin,
            direction: direction
        }
    }
}

fn color(ray: Ray) -> Vector3 {
    let unit_dir = ray.direction.normalize();
    let t = 0.5 * unit_dir.y + 1.0;
    (1.0 - t) * Vector3::one() + t * Vector3::new(0.5, 0.7, 1.0)
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

    for j in (0..ny-1).rev() {
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
