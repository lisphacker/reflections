
use crate::vecmath::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn at(self: Self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin:            Vector3,
    lower_left_corner: Vector3,
    horizontal:        Vector3,
    vertical:          Vector3
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            origin:            Vector3::zero(),
            lower_left_corner: Vector3::new(-2.0, 1.0, 1.0),
            horizontal:        Vector3::new(4.0, 0.0, 0.0),
            vertical:          Vector3::new(0.0, 2.0, 0.0)
        }
    }

    pub fn ray(self: Self, u: f32, v: f32) -> Ray {
        Ray {
            origin:    self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
        }
    }   
}
