pub mod point;

use std::io::{self, Write};
use raytracer::color::Color;

fn main() {
    create_image(256, 256);
}

fn create_image(height: i32, width: i32) -> () {
    println!("P3\n{width} {height}\n255");

    for j in (0..height).rev() {
        eprint!("\r Scanlines remaining {j}");
        io::stderr().flush().unwrap();
        for i in 0..width {
            let color = Color {
                r: i as f32 / (width - 1) as f32,
                g: j as f32 / (height -1) as f32,
                b: 0.25,
            };

            println!("{color}");
        } 
    }
}