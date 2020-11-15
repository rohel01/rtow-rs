use std::ops::{Deref, DerefMut, RangeInclusive};

use crate::data::{Point3, Vec3};
use crate::ray::Ray;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, outward_normal: &Vec3, t: f32, r: &Ray) -> Self {
        let front_face = Vec3::dot(&r.dir, outward_normal).is_sign_negative();
        let normal = if front_face {
            *outward_normal
        } else {
            -outward_normal
        };

        assert_eq!(p, r.at(t), "Inconsistent geometry !");

        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
    pub fn p(&self) -> &Point3 {
        &self.p
    }
}

pub type HitRange = RangeInclusive<f32>;

pub trait Hittable {
    fn hit(&self, r: &Ray, range: HitRange) -> Option<HitRecord>;
}

pub struct HittableList(Vec<Box<dyn Hittable>>);

impl HittableList {
    pub fn new() -> Self {
        HittableList { 0: vec![] }
    }
}

impl Deref for HittableList {
    type Target = Vec<Box<dyn Hittable>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HittableList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, range: HitRange) -> Option<HitRecord> {
        let mut closest_so_far: f32 = *range.end();
        let mut hit_record: Option<HitRecord> = Option::None;

        self.0.iter().for_each(|hittable| {
            let reduced_range = HitRange::new(*range.start(), closest_so_far);
            if let Some(rec) = hittable.hit(r, reduced_range) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        });

        hit_record
    }
}
