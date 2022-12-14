#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

use bevy::prelude::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0 } }
    pub fn sqr_magnitude(&self) -> f32 { self.x * self.x + self.y * self.y }
    pub fn magnitude(&self) -> f32 { self.sqr_magnitude().sqrt() }
    pub fn normalized(&self) -> Self { *self / self.magnitude() }
    pub fn dot(a: Self, b: Self) -> f32 { a.x * b.x + a.y * b.y }
    pub fn as_vec3(self, z: f32) -> Vec3 { Vec3::new(self.x, self.y, z) }
}

impl Default for Vector {
    fn default() -> Self { Self::zero() }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs  }
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            Self { x: 0.0, y: 0.0 }
        } else {
            Self { x: self.x / rhs, y: self.y / rhs }
        }
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) {
        if rhs == 0.0 {
            self.x = 0.0;
            self.y = 0.0;
        } else {
            self.x /= rhs;
            self.y /= rhs;
        }
    }
}
