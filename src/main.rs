use std::io::stdout;

use indicatif::ProgressIterator;

use crate::data::{Color, Point3, Vec3};
use crate::ray::Ray;

mod data;
mod ray;

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> bool {
    let oc = r.orig - center;
    let a = Vec3::dot(&r.dir, &r.dir);
    let b = 2.0 * Vec3::dot(&oc, &r.dir);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > f32::EPSILON
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = Vec3::unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400i32;
    let image_height = ((image_width as f32) / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0f32;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0f32;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev().progress() {
        for i in 0..image_width {
            let u = (i as f32) / ((image_width - 1) as f32);
            let v = (j as f32) / ((image_height - 1) as f32);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);

            data::write_color(&mut stdout(), &pixel_color);
        }
    }
}
