use rand::Rng;

use crate::{ray::{HitRecord, Ray}, color::Color, point::random_unit_vector};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Color)>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LambertianMaterial {
    pub albedo: Color,
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Color)> {
        let scatter_dir = hit.normal + random_unit_vector(rng);
        let scattered = Ray {origin: hit.point, direction: scatter_dir};

        // catch degenerate scatter direction
        if scatter_dir.near_zero() {
            return Some((Ray {origin: hit.point, direction: hit.normal}, self.albedo));
        }
        return Some((scattered, self.albedo));
    }
}