use rand::Rng;
use std::ops::{Deref, DerefMut, RangeInclusive};

use crate::data::{Color, Point3, Vec3};
use crate::dielectric::Dielectric;
use crate::lambertian::Lambertian;
use crate::material::Material;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::sphere::Sphere;

pub struct HitRecord<'a> {
    p: Point3,
    normal: Vec3,
    material: &'a Box<dyn Material + Send + Sync>,
    t: f32,
    front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        material: &'a Box<dyn Material + Send + Sync>,
        outward_normal: &Vec3,
        t: f32,
        r: &Ray,
    ) -> Self {
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
            material,
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
    pub fn material(&self) -> &Box<dyn Material + Send + Sync> {
        &self.material
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub type HitRange = RangeInclusive<f32>;

pub trait Hittable {
    fn hit(&self, r: &Ray, range: HitRange) -> Option<HitRecord>;
}

pub struct HittableList<'a>(Vec<Box<dyn Hittable + Sync + Send + 'a>>);

impl HittableList<'_> {
    pub fn new() -> Self {
        HittableList { 0: vec![] }
    }

    pub fn random_scene() -> Self {
        let mut world = Self::new();
        let material_ground = Box::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));

        let sphere_ground = Box::new(Sphere::new(
            Point3::new(0.0, -1000.0, -1.0),
            1000.0,
            material_ground,
        ));
        (*world).push(sphere_ground);

        let mut rng = rand::thread_rng();
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = rng.gen::<f32>();
                let center = Point3::new(
                    a as f32 + 0.9 * rng.gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rng.gen::<f32>(),
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Color::random() * Color::random();
                        let sphere_material = Box::new(Lambertian::new(&albedo));
                        let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                        (*world).push(sphere);
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Color::random_bounded(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0f32, 0.5f32);
                        let sphere_material = Box::new(Metal::new(&albedo, fuzz));
                        let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                        (*world).push(sphere);
                    } else {
                        let sphere_material = Box::new(Dielectric::new(1.5));
                        let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                        (*world).push(sphere);
                    }
                }
            }
        }

        let material1 = Box::new(Dielectric::new(1.5));
        let sphere1 = Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));
        (*world).push(sphere1);

        let material2 = Box::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
        let sphere2 = Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));
        (*world).push(sphere2);

        let material3 = Box::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
        let sphere3 = Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));
        (*world).push(sphere3);

        world
    }
}

impl<'a> Deref for HittableList<'a> {
    type Target = Vec<Box<dyn Hittable + Sync + Send + 'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HittableList<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Hittable for HittableList<'_> {
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
