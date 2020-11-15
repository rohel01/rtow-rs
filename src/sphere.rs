use crate::data::{Point3, Vec3};
use crate::hittable::{HitRange, HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Box<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f32, m: Box<dyn Material + Send + Sync>) -> Self {
        Sphere {
            center: cen,
            radius: r,
            material: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, range: HitRange) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(&oc, &r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < f32::EPSILON {
            return Option::None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the neared root that lies in the acceptable range
        let roots = vec![(-half_b - sqrtd) / a, (-half_b + sqrtd) / a];
        if let Some(valid_root) = roots
            .iter()
            .find(|root| range.start() <= *root && *root <= range.end())
        {
            let valid_hitpoint = r.at(*valid_root);
            let outward_normal = (valid_hitpoint - self.center) / self.radius;
            Option::from(HitRecord::new(
                valid_hitpoint,
                &self.material,
                &outward_normal,
                *valid_root,
                r,
            ))
        } else {
            Option::None
        }
    }
}
