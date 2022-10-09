use rand::Rng;
use rand::distributions::WeightedError;
use rand::seq::SliceRandom;

use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::config::Config;
use raytracer::material::{LambertianMaterial, Metal, Dialectric, Material};
use raytracer::point::Point3;
use raytracer::raytracer::render_image;
use raytracer::sphere::Sphere;
// use raytracer::torus::Torus;
use raytracer::ray::HittableList;

fn main() {
    let scene = Config {
        width: 800,
        height: 600,
        samples_per_pixel: 200,
        max_depth: 50,
    };
    let aspect_ratio = scene.width as f64 / scene.height as f64;


    let objects = match create_random_world() {
        Err(_) => panic!("Could not create world"),
        Ok(world) => world,
    };
    
    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.05,
    );
    render_image(&scene, &camera, &objects);
}

fn create_random_world() -> Result<HittableList, WeightedError> {
    let mut objects = HittableList::new();
    let mut rng = rand::thread_rng();
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

    for a in -10..10 {
        for b in -10..10 {
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
