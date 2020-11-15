use crate::data::{Color, Vec3};
use crate::hittable::HitRecord;
use crate::material::{Material, ScatteredRecord};
use crate::ray::Ray;

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Self { albedo: *a }
    }
}

impl Material for Lambertian {
    fn scatter<'a>(&self, _r_in: &Ray, rec: &HitRecord<'a>) -> Option<ScatteredRecord> {
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
