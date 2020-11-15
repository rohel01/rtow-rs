use std::io::stdout;

use indicatif::ProgressIterator;
use rand::Rng;

use crate::camera::Camera;
use crate::data::{Color, Point3, Vec3};
use crate::dielectric::Dielectric;
use crate::hittable::{HitRange, Hittable, HittableList};
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::sphere::Sphere;

mod camera;
mod data;
mod dielectric;
mod hittable;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;

const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: &mut impl Iterator<Item = u8>) -> Color {
    if depth.next().is_some() {
        if let Some(rec) = world.hit(r, HitRange::new(0.001, f32::INFINITY)) {
            if let Some(scatter_record) = rec.material().scatter(r, &rec) {
                return scatter_record.attenuation * ray_color(&scatter_record.ray, world, depth);
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = Vec3::unit_vector(&r.dir);
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    } else {
        Color::new(0.0, 0.0, 0.0)
    }
}

fn main() {
    // Image
    let image_width = 400i32;
    let image_height = ((image_width as f32) / ASPECT_RATIO) as i32;
    let samples_per_pixel = 100i32;
    let max_depth = 50u8;

    // World
    let material_ground = Lambertian::new(&Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(&Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(&Color::new(0.8, 0.6, 0.2), 0.0);

    let sphere_ground = Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        &material_ground,
    ));
    let sphere_center = Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        &material_center,
    ));
    let sphere_left = Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        &material_left,
    ));
    let sphere_right = Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        &material_right,
    ));

    let mut world = HittableList::new();
    (*world).push(sphere_ground);
    (*world).push(sphere_center);
    (*world).push(sphere_left);
    (*world).push(sphere_right);

    // Camera
    let cam = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
    );

    //Render
    println!("P3\n{} {}\n255", image_width, image_height);

    let mut rng = rand::thread_rng();
    for j in (0..image_height).rev().progress() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / ((image_width - 1) as f32);
                let v = (j as f32 + rng.gen::<f32>()) / ((image_height - 1) as f32);

                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, &mut (0..max_depth));
            }

            data::write_color(&mut stdout(), &pixel_color, samples_per_pixel);
        }
    }
}
