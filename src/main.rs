mod colour;
mod vec3;

use colour::Colour;
use std::io;

fn main() {
    // Image size
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);

        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25 as f32;

            let pixel_colour = Colour::new(r, g, b);
            colour::write_colour(&mut io::stdout(), pixel_colour);
        }
    }

    eprint!("\nDone.\n");
}
