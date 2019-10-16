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
        Vector3::new(
            self * multiplier.x,
            self * multiplier.y,
            self * multiplier.z,
        )
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

    fn copy(v: &Self) -> Self {
        return Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        };
    }

    fn length(self: Self) -> f32 {
        return (self * self).sqrt();
    }

    pub fn normalize(self: Self) -> Self {
        self / self.length()
    }
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
