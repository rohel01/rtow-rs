use std::ops::Deref;
use std::ops::{Add, Div, Mul, Sub};

use std::io::Write;

const EPSILON: f32 = 1e-8;

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(u: &Self, v: &Self) -> f32 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Self, v: &Self) -> Self {
        Self::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }

    pub fn unit_vector(v: Self) -> Self {
        v / v.length()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        if other.abs() < EPSILON {
            panic!("Invalid division by zero!");
        }

        // TODO Comprendre comment je peux rendre ca commutatif
        self * 1.0 / other
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

//TODO Utiliser le pattern newtype pour distinguer Point3, Color et Vec3
// sans dupliquer ou router manuellement les methodes (Dered?)
pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn write_color(w: &mut Write, pixel_color: Color) {
    let ir = (255.999 * pixel_color.x) as i32;
    let ig = (255.999 * pixel_color.y) as i32;
    let ib = (255.999 * pixel_color.z) as i32;

    writeln!(w, "{} {} {}", ir, ig, ib).expect("Failed to write to target stream!");
}
