pub mod point;

use std::io::{self, Write};
use rand::Rng;

use raytracer::color::Color;
use raytracer::material::{LambertianMaterial, Material};
use raytracer::point::{Point3, Vector3};
use raytracer::sphere::Sphere;
// use raytracer::torus::Torus;
use raytracer::ray::{Hittable, HittableList, Ray};

fn ray_color(ray: &Ray, objects: &HittableList, max_depth: i32) -> Color {
    let mut cur_ray = *ray;
    let mut cur_color = Color {r: 0.0, g: 0.0, b: 0.0 };
    let mut color_coef = Color {r: 1.0, g: 1.0, b: 1.0 };

    for _ in 0..max_depth {
        match objects.hit(&cur_ray, 0.0001, f64::INFINITY) {
            Some(hit) => {
                match hit.material.scatter(&cur_ray, &hit) {
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
    cur_color

    
}

fn main() {
    create_image();
}



fn create_image() -> () {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut rng = rand::thread_rng();


    // World
    let material = LambertianMaterial {
        albedo: Color {r: 0.5, g: 0.5, b: 0.5}
    };
    let objects = HittableList {
        objects: vec![
            Box::new(Sphere {
                center: Point3 {x: 0.0, y: 0.0, z: -1.0},
                radius: 0.5,
                material: material,
            }),
            // Box::new(Sphere {center: Point3 {x: 0.8, y: -0.3, z: -0.9}, radius: 0.2}),
            // Box::new(Torus {center: Point3 {x: -0.5, y: -0.3, z: -1.0}, a: 0.3, b: 0.1}),
            Box::new(Sphere {
                center: Point3 {x: 0.0, y: -100.5, z: -1.0},
                radius: 100.0,
                material: material,
            }),
        ]
    };
    let camera = get_camera(aspect_ratio);


    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).rev() {
        eprint!("\r Scanlines remaining {j}");
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let mut color = Color {r: 0.0, g: 0.0, b: 0.0};
            for _ in 0..samples_per_pixel {
                let ray = camera.get_ray(
                    (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64,
                    (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64
                );
                color = color + ray_color(&ray, &objects, max_depth);
            }
            color = (color / (samples_per_pixel as f64)).gamma_correct();


            println!("{color}");
        } 
    }
}

struct Camera {
    origin: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Vector3,
}

fn get_camera(aspect_ratio: f64) -> Camera {
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let origin = Point3 {x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vector3 { x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vector3 { x: 0.0, y: viewport_height, z: 0.0};
    let distance = Vector3 { x: 0.0, y: 0.0, z: focal_length };
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - distance;

    Camera {
        origin,
        horizontal,
        vertical,
        lower_left_corner,
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin,
        }
    }
}