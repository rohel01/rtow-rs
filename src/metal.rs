use crate::data::{Color, Vec3};
use crate::hittable::HitRecord;
use crate::material::{Material, ScatteredRecord};
use crate::ray::Ray;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(a: &Color, f: f32) -> Self {
        let fuzz = if f < 1.0 { f } else { 1.0 };

        Self { albedo: *a, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRecord> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&r_in.dir), rec.normal());
        let scattered = Ray::new(
            *rec.p(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        if Vec3::dot(&scattered.dir, rec.normal()) > 0.0 {
            Option::Some(ScatteredRecord {
                attenuation: self.albedo,
                ray: scattered,
            })
        } else {
            Option::None
        }
    }
}
