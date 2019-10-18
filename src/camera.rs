use crate::vecmath::*;
use crate::rays::*;

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
            lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
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
