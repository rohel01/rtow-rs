use crate::data::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct ScatteredRecord {
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRecord>;
}
