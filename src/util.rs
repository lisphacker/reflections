use rand::Rng;
use crate::vecmath::*;

pub fn random() -> f32 {
    rand::thread_rng().gen()
}

pub fn random_in_unit_sphere() -> Vector3 {
    let mut p = Vector3::one() * 100.0;

    while p * p > 1.0 {
        p = Vector3::new(random(), random(), random()) * 2.0 - 1.0;
    };

    p
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
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
