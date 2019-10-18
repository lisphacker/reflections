use crate::vecmath::*;
use crate::materials::*;
use std::rc::Rc;

pub struct HitGeometry {
    pub t: f32,
    pub point: Vector3,
    pub normal: Vector3,
}

pub struct HitResult {
    pub geometry: HitGeometry,
    pub material: Rc<Box<dyn Material>>
}

pub trait Hittable {
    fn hit(self: &Self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitResult>;
}

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
