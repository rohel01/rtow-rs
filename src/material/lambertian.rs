use crate::color::Color;
use crate::geometry::hittable::HitRecord;
use crate::geometry::ray::Ray;
use crate::geometry::Vec3;
use crate::material::{Material, ScatteredRecord};

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Self { albedo: *a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRecord> {
        let mut scatter_direction = rec.normal() + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = *rec.normal()
        }
        let scattered = Ray::new(*rec.p(), scatter_direction);

        Option::Some(ScatteredRecord {
            attenuation: self.albedo,
            ray: scattered,
        })
    }
}
