use std::io::stdout;

use indicatif::ProgressIterator;
use rand::Rng;

use crate::camera::Camera;
use crate::data::{Color, Point3, Vec3};
use crate::hittable::{HitRange, Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;

mod camera;
mod data;
mod hittable;
mod ray;
mod sphere;

const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: &mut impl Iterator<Item = u8>) -> Color {
    if depth.next().is_some() {
        if let Some(rec) = world.hit(r, HitRange::new(0.001, f32::INFINITY)) {
            let target = rec.p() + Vec3::random_in_hemisphere(rec.normal());
            return 0.5 * ray_color(&Ray::new(*rec.p(), target - rec.p()), world, depth);
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
    let mut world = HittableList::new();
    (*world).push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    (*world).push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

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
