#![allow(dead_code)]

use crate::vmath::utils::Print;
use std::fmt::Formatter;

#[derive(Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: std::clone::Clone + std::marker::Copy> Vector3<T> {
    pub fn new(x_: T, y_: T, z_: T) -> Vector3<T> {
        Vector3 {
            x: x_,
            y: y_,
            z: z_,
        }
    }
}

impl Vector3<u16> {
    pub fn clamp(&mut self) -> Vector3<u8> {
        let mut max = 0;
        if self.x > max {
            max = self.x;
        }
        if self.y > max {
            max = self.y;
        }
        if self.z > max {
            max = self.z;
        }

        if max > 255 {
            return Vector3::new(
                (self.x / max) as u8 * 255,
                (self.y / max) as u8 * 255,
                (self.z / max) as u8 * 255,
            );
        } else {
            return Vector3::new(self.x as u8, self.y as u8, self.z as u8);
        }
    }
}

impl Vector3<f32> {
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }

    pub fn magnitude(&mut self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl<T: std::ops::Add<Output = T> + std::marker::Copy> std::ops::Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: std::ops::Sub<Output = T> + std::marker::Copy> std::ops::Sub<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: std::marker::Copy + std::ops::Mul<Output = T>> std::ops::Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: std::marker::Copy + std::ops::Mul<Output = T>> std::ops::Mul<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T: std::marker::Copy + std::ops::Div<Output = T>> std::ops::Div<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl<T> Print for Vector3<T> {}
