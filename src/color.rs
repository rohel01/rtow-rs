use std::io::Write;
use std::ops::RangeInclusive;

use crate::geometry::Vec3;

pub type Color = Vec3;

pub type ValueRange = RangeInclusive<f32>;

fn clamp(val: f32, range: &ValueRange) -> f32 {
    if val < *range.start() {
        *range.start()
    } else if val > *range.end() {
        *range.end()
    } else {
        val
    }
}

pub fn write_color(w: &mut dyn Write, pixel_color: &Color, samples_per_pixel: i32) {
    let clamp_range = 0.0f32..=0.999f32;

    // Divide the color by the number of samples
    let scale = 1.0 / (samples_per_pixel as f32);
    let scaled_color = scale * pixel_color;

    // Apply gamma 2 correction
    let corrected_color = scaled_color.sqrt();

    let ir = (256.0 * clamp(corrected_color.x, &clamp_range)) as i32;
    let ig = (256.0 * clamp(corrected_color.y, &clamp_range)) as i32;
    let ib = (256.0 * clamp(corrected_color.z, &clamp_range)) as i32;

    writeln!(w, "{} {} {}", ir, ig, ib).expect("Failed to write to target stream!");
}
