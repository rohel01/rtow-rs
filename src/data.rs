use std::io::Write;
use std::ops::{Add, Div, Mul, Neg, RangeInclusive, Sub};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
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

    pub fn unit_vector(v: &Self) -> Self {
        v / v.length()
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: &Vec3) -> Self::Output {
        &self + other
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        self + &other
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: &Vec3) -> Self::Output {
        &self - other
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        self - &other
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        self * (&other)
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Self::Output {
        other * self
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        other * self
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Self::Output {
        if other.abs() < f32::EPSILON {
            panic!("Invalid division by zero!");
        }

        1.0 / other * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        (&self).div(other)
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

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

//TODO Utiliser le pattern newtype pour distinguer Point3, Color et Vec3
// sans dupliquer ou router manuellement les methodes (Dered?)
pub type Point3 = Vec3;
pub type Color = Vec3;

pub type ValueRange = RangeInclusive<f32>;

fn clamp(val: f32, range: &ValueRange) -> f32 {
    if val < *range.start() {
        *range.start()
    } else if val > *range.end() {
        *range.end()
    } else {
        val
    }
}

pub fn write_color(w: &mut dyn Write, pixel_color: &Color, samples_per_pixel: i32) {
    // Divide the color by the number of samples
    let scale = 1.0 / (samples_per_pixel as f32);

    let clamp_range = 0.0f32..=0.999f32;

    let scaled_color = scale * pixel_color;
    let ir = (256.0 * clamp(scaled_color.x, &clamp_range)) as i32;
    let ig = (256.0 * clamp(scaled_color.y, &clamp_range)) as i32;
    let ib = (256.0 * clamp(scaled_color.z, &clamp_range)) as i32;

    writeln!(w, "{} {} {}", ir, ig, ib).expect("Failed to write to target stream!");
}
