use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Add for Vector3 {
    type Output = Self;

    fn add(self: Self, operand: Self) -> Self {
        Vector3::new(self.x + operand.x, self.y + operand.y, self.z + operand.z)
    }
}

impl ops::Add<f32> for Vector3 {
    type Output = Self;

    fn add(self: Self, operand: f32) -> Self {
        Vector3::new(self.x + operand, self.y + operand, self.z + operand)
    }
}

impl ops::Add<Vector3> for f32 {
    type Output = Vector3;

    fn add(self: Self, operand: Vector3) -> Vector3 {
        operand + self
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self: Self, operand: Self) -> Self {
        Vector3::new(self.x - operand.x, self.y - operand.y, self.z - operand.z)
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self: Self, multiplier: f32) -> Self {
        Vector3::new(
            self.x * multiplier,
            self.y * multiplier,
            self.z * multiplier,
        )
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self: Self, multiplier: Vector3) -> Vector3 {
        multiplier * self
    }
}

impl ops::Mul for Vector3 {
    type Output = f32;

    fn mul(self: Self, multiplier: Self) -> f32 {
        self.x * multiplier.x + self.y * multiplier.y + self.z * multiplier.z
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Self;

    fn div(self: Self, divisor: f32) -> Self {
        self * (1.0 / divisor)
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Self {
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    // fn copy(v: &Self) -> Self {
    //     return Vector3 {
    //         x: v.x,
    //         y: v.y,
    //         z: v.z,
    //     };
    // }

    fn length(self: Self) -> f32 {
        return (self * self).sqrt();
    }

    pub fn normalize(self: Self) -> Self {
        self / self.length()
    }
}

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
