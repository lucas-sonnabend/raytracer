use rand::Rng;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::color::{Color, Pixel, Pixels};
use crate::config::Config;
use crate::ray::{Hittable, HittableList, Ray};

fn ray_color(ray: &Ray, objects: &HittableList, max_depth: u32) -> Color {
    let mut cur_ray = *ray;
    let mut cur_color = Color {r: 0.0, g: 0.0, b: 0.0 };
    let mut color_coef = Color {r: 1.0, g: 1.0, b: 1.0 };

    for _ in 0..max_depth {
        match objects.hit(&cur_ray, 0.0001, f64::INFINITY) {
            Some((hit, material)) => {
                match material.scatter(&cur_ray, &hit) {
                    Some((new_ray, attenuation)) => {
                        cur_ray = new_ray;
                        color_coef = color_coef * attenuation;
                    }
                    None => {break;}
                }
            }
            None => {
                let unit_direction = ray.direction.unit_direction();
                let t = 0.5 * (unit_direction.y + 1.0);
                let start_color = Color { r: 1.0, g: 1.0, b: 1.0};
                let end_color =  Color { r: 0.5, g: 0.7, b: 1.0};
                cur_color =  (start_color * (1.0 - t) + end_color * t) * color_coef;
                break;
            }
        }
    };
    return cur_color;
}


pub fn render_image(scene: &Config, camera: &Camera, objects: &HittableList) {
    let mut pixels = vec![(0 as u8, 0 as u8, 0 as u8); scene.width * scene.height as usize];

    let bands = pixels.par_chunks_mut(scene.width).enumerate();
    bands.for_each(|(i, pixels)|render_line(i, &scene, pixels, &camera, &objects));

    print_image(scene.width, scene.height, &pixels);
}


fn render_line(
    line_no: usize,
    scene: &Config,
    pixels: &mut [Pixel],
    camera: &Camera,
    objects: &HittableList,
) {
    let mut rng = rand::thread_rng();
    let j = scene.height - 1 - line_no;
    for i in 0..scene.width {
        let mut color = Color {r: 0.0, g: 0.0, b: 0.0};
        for _ in 0..scene.samples_per_pixel {
            let ray = camera.get_ray(
                (i as f64 + rng.gen_range(0.0..1.0)) / (scene.width - 1) as f64,
                (j as f64 + rng.gen_range(0.0..1.0)) / (scene.height - 1) as f64
            );
            color = color + ray_color(&ray, objects, scene.max_depth);
        }
        color = (color / (scene.samples_per_pixel as f64)).gamma_correct();
        pixels[i] = color.to_simple();
    }
}

fn print_image(image_width: usize, image_height: usize, pixels: &Pixels) {
    println!("P3\n{image_width} {image_height}\n255");
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel = pixels[j * image_width + i];
            println!("{} {} {}", pixel.0, pixel.1, pixel.2);
        }
    }
}