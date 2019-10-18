
use crate::vecmath::*;
use crate::camera::*;

#[derive(Debug, Copy, Clone)]
pub struct HitResult {
    pub t: f32,
    pub point: Vector3,
    pub normal: Vector3,
}

pub trait Hittable {
    fn hit(self: &Self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitResult>;
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub centre: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(centre: Vector3, radius: f32) -> Self {
        Sphere {
            centre: centre,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(self: &Self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let oc = ray.origin - self.centre;
        let a = ray.direction * ray.direction;
        let b = oc * ray.direction;
        let c = (oc * oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.at(temp);
                Some(HitResult {
                    t: temp,
                    point: p,
                    normal: (p - self.centre) / self.radius
                })
            }
            else {
                let temp = (-b + discriminant.sqrt()) / a;
                if temp < t_max && temp > t_min {
                    let p = ray.at(temp);
                    Some(HitResult {
                        t: temp,
                        point: p,
                        normal: (p - self.centre) / self.radius
                    })
                }
                else {
                    None
                }
            }
        }
        else {
            None
        }
    }
}

pub struct HittableList {
    hittables: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            hittables: Vec::new()
        }
    }

    pub fn add(self: &mut Self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable)
    }
}

// //impl<'a> Copy for HittableList<'a> {}
// impl<'a> Clone for HittableList<'a> {
//     fn clone(self: &Self) -> Self {
//         let &hittables: &mut Vec<&'a dyn Hittable> = &mut Vec::new();
//         HittableList {
//             hittables: hittables
//         }
//     }
// }

impl Hittable for HittableList {
    fn hit(self: &Self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitResult> {
        let mut hit = false;
        let mut hit_result: HitResult = HitResult { 
            t:      0.0,
            point:  Vector3::zero(),
            normal: Vector3::zero()
        };
        for hittable in self.hittables.iter() {
            let optional_result = hittable.hit(ray, t_min, t_max);
            match optional_result {
                Some(result) => if !hit || result.t < hit_result.t {
                    hit = true;
                    hit_result = result;
                }
                None => {}
            }
        }
        
        if hit {
            Some(hit_result)
        }
        else {
            None
        }
    }
}
