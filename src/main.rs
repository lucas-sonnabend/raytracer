pub mod point;

use std::io::{self, Write};
use rand::Rng;
use rand::distributions::WeightedError;
use rand::seq::SliceRandom;

use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::material::{LambertianMaterial, Metal, Dialectric, Material};
use raytracer::point::{Point3};
use raytracer::sphere::Sphere;
// use raytracer::torus::Torus;
use raytracer::ray::{Hittable, HittableList, Ray};

fn ray_color(ray: &Ray, objects: &HittableList, max_depth: i32) -> Color {
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
    cur_color

    
}

fn main() {
    create_image();
}


fn create_random_world() -> Result<HittableList, WeightedError> {
    let mut objects = HittableList::new();
    let mut rng = rand::thread_rng();
    // TODO: add lots of spheres randomly
    let ground_material = LambertianMaterial::new(
        Color {r: 0.5, g: 0.5, b: 0.5}
    );
    let ground = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(ground_material),
    );
    objects.add(Box::new(ground));

    let material_choices = [
        ("diffuse", 16), ("metal", 3), ("glass", 1)
    ];

    for a in -5..5 {
        for b in -5..5 {
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.gen_range(0.0..1.0),
            );
            let point = Point3::new(4.0, 0.2, 0.0);

            if (center - point).length() > 0.9 {
                let material_choice = material_choices.choose_weighted(&mut rng, |item| item.1);
                let material = match material_choice {
                    Ok(("diffuse", _)) => {
                        let albedo = Color::random() * Color::random();
                        Box::new(LambertianMaterial::new(albedo)) as Box<dyn Material>
                    },
                    Ok(("metal", _)) => {
                        let albedo = Color::random_with_limits(0.5, 1.0);
                        let fuzz = rng.gen_range(0.0..0.5);
                        Box::new(Metal::new(albedo, fuzz)) as Box<dyn Material>
                    },
                    Ok(("glass", _)) => {
                        Box::new(Dialectric::new(1.5)) as Box<dyn Material>
                    },
                    Ok((_mat, _)) => {panic!("unknown material")},
                    Err(e) => return Err(e),
                };
                objects.add(Box::new(Sphere::new(center, 0.2, material)));
            }

        }
    }
    let mat1 = Box::new(Dialectric::new(1.5));
    objects.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Box::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    objects.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    objects.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    return Ok(objects);
}

pub fn create_simple_world() -> HittableList {
    let ground_material = LambertianMaterial::new(
        Color {r: 0.8, g: 0.8, b: 0.0}
    );
    let material_center = Dialectric::new(1.5);
    let material_left = Metal::new(
        Color {r: 0.8, g: 0.8, b: 0.8}, 0.3
    );
    let material_right = Metal::new(
        Color {r: 0.8, g: 0.6, b: 0.2}, 1.0,
    );

    let objects = HittableList {
        objects: vec![
            Box::new(Sphere {
                center: Point3 {x: 0.0, y: -100.5, z: -1.0},
                radius: 100.0,
                material: Box::new(ground_material),
            }),
            Box::new(Sphere {
                center: Point3 {x: 0.0, y: 0.0, z: -1.0},
                radius: 0.5,
                material: Box::new(material_center),
            }),
            Box::new(Sphere {
                center: Point3 {x: 0.0, y: 0.0, z: -1.0},
                radius: -0.45,
                material: Box::new(material_center),
            }),
            Box::new(Sphere {
                center: Point3 {x: -1.0, y: 0.0, z: -1.0},
                radius: 0.5,
                material: Box::new(material_left),
            }),
            Box::new(Sphere {
                center: Point3 {x: 1.0, y: 0.0, z: -1.0},
                radius: 0.5,
                material: Box::new(material_right),
            }),
        ]
    };
    return objects;
}

fn create_image() -> Result<(), WeightedError> {
    let aspect_ratio = 4.0 / 3.0;
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut rng = rand::thread_rng();


    let objects = create_random_world()?;
    
    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
    );

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
    };
    return Ok(());
}
