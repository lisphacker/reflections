use crate::vecmath::*;
use crate::rays::*;
use crate::util::*;

pub struct ScatterResult {
    pub attenuation:   Vector3,
    pub scattered_ray: Ray
}

pub trait Material {
    fn scatter(self: &Self, input_ray: Ray, hit_result: &HitGeometry) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Vector3
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Self {
        Lambertian {
            albedo: albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(self: &Self, input_ray: Ray, hit_result: &HitGeometry) -> Option<ScatterResult> {
        let target = hit_result.point + hit_result.normal + random_in_unit_sphere();
        Some(ScatterResult {
            attenuation: self.albedo,
            scattered_ray: Ray::new(hit_result.point, target - hit_result.point)
        })
    }
}

pub struct Metal {
    albedo: Vector3,
    fuzz:   f32
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f32) -> Self {
        Metal {
            albedo: albedo,
            fuzz:   if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(self: &Self, input_ray: Ray, hit_geometry: &HitGeometry) -> Option<ScatterResult> {
        let reflected = reflect(input_ray.direction.normalize(), hit_geometry.normal);
        let scattered_ray = Ray::new(hit_geometry.point, reflected + self.fuzz * random_in_unit_sphere());
        if scattered_ray.direction * hit_geometry.normal > 0.0 {
            Some(ScatterResult {
                attenuation:   self.albedo,
                scattered_ray: scattered_ray
            })
        }
        else {
            None
        }
        
    }
}
