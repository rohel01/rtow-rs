use std::io::stdout;

use data::Color;
use indicatif::ProgressBar;

mod data;

fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    let bar = ProgressBar::new((image_height * image_width) as u64);
    for i in (0..image_height).rev() {
        for j in 0..image_width {
            let pixel_color = Color::new(
                (i as f32) / ((image_width - 1) as f32),
                (j as f32) / ((image_height - 1) as f32),
                0.25,
            );

            data::write_color(&mut stdout(), pixel_color);
            bar.inc(1);
        }
    }
    bar.finish();
}
