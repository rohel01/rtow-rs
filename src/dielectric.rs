use rand::Rng;

use crate::data::{Color, Vec3};
use crate::hittable::HitRecord;
use crate::material::{Material, ScatteredRecord};
use crate::ray::Ray;

pub struct Dielectric {
    ir: f32, // Index of Refraction
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0_2 = r0 * r0;

        r0_2 + (1.0 - r0_2) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter<'a>(&self, r_in: &Ray, rec: &HitRecord<'a>) -> Option<ScatteredRecord> {
        let refraction_ratio = if rec.front_face() {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(&r_in.dir);
        let cos_theta = Vec3::dot(&-unit_direction, rec.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = rand::thread_rng();
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>()
        {
            Vec3::reflect(&unit_direction, rec.normal())
        } else {
            Vec3::refract(&unit_direction, rec.normal(), refraction_ratio)
        };

        let scattered = Ray::new(*rec.p(), direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);

        Option::Some(ScatteredRecord {
            attenuation,
            ray: scattered,
        })
    }
}
