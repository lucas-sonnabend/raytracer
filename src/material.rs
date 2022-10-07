use std::fmt;

use crate::{ray::{HitRecord, Ray}, color::Color, point::{random_unit_vector, random_vector_in_unit_sphere}};

pub trait Material: fmt::Debug{
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LambertianMaterial {
    albedo: Color,
}

impl LambertianMaterial {
    pub fn new(albedo: Color) -> Self {
        Self {albedo}
    }
    pub fn default() -> Self {
        Self {albedo: Color {r: 0.0, g: 0.0, b: 0.0}}
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_dir = hit.normal + random_unit_vector();
        let scattered = Ray {origin: hit.point, direction: scatter_dir};

        // catch degenerate scatter direction
        if scatter_dir.near_zero() {
            return Some((Ray {origin: hit.point, direction: hit.normal}, self.albedo));
        }
        return Some((scattered, self.albedo));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let safe_fuzz = match fuzz {
            x if x < 0.0 => 0.0,
            x if x > 1.0 => 1.0,
            x => x,
        };
        return Self { albedo, fuzz: safe_fuzz};
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray_in.direction.unit_direction().reflect(&hit.normal);
        let scattered = Ray {origin: hit.point, direction: reflected  + random_vector_in_unit_sphere() * self.fuzz};
        if scattered.direction.dot_product(&hit.normal) > 0.0 {
            return Some((scattered, self.albedo));
        } else {
            return None;
        }
    }
}
