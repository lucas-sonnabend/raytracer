pub mod point;

use std::io::{self, Write};
use raytracer::color::Color;
use raytracer::point::{Point3, Vector3};
use raytracer::ray::Ray;

fn ray_color(ray: &Ray) -> Color {
    let sphere_center = Point3 {x: 0.0, y: 0.0, z: -1.0};
    let sphere_radius = 0.5;
    let t = sphere_color(&sphere_center, sphere_radius, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vector3 {x: 0.0, y: 0.0, z: -1.0}).unit_direction();
        return Color { r: n.x + 1.0, g: n.y + 1.0, b: n.z + 1.0} * 0.5;
    }
    let unit_direction = ray.direction.unit_direction();
    let t = 0.5 * (unit_direction.y + 1.0);
    let start_color = Color { r: 1.0, g: 1.0, b: 1.0};
    let end_color =  Color { r: 0.5, g: 0.7, b: 1.0};
    return start_color * (1.0 - t) + end_color * t;
}

fn sphere_color(center: &Vector3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot_product(&ray.direction);
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0
    } else {
        return (-half_b - f64::sqrt(discriminant) ) / a
    }
}

fn main() {
    create_image();
}



fn create_image() -> () {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3 { x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vector3 { x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vector3 { x: 0.0, y: viewport_height, z: 0.0};
    let distance = Vector3 { x: 0.0, y: 0.0, z: focal_length };
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - distance;

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).rev() {
        eprint!("\r Scanlines remaining {j}");
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray {
                origin: origin,
                direction: lower_left_corner + horizontal*u + vertical*v - origin,
            };
            let color = ray_color(&ray);

            println!("{color}");
        } 
    }
}