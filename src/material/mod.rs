use crate::color::Color;
use crate::geometry::hittable::HitRecord;
use crate::geometry::ray::Ray;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub struct ScatteredRecord {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRecord>;
}
