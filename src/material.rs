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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dialectric {
    index_of_refraction: f64,
}

impl Dialectric {
    pub fn new(index_of_refraction: f64) -> Self {
        return Self { index_of_refraction };
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color {r: 1.0, g: 1.0, b: 1.0};
        let refraction_ratio = match hit.front_face {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction,
        };
        let unit_direction = ray_in.direction.unit_direction();
        let cos_theta = f64::min((unit_direction * -1.0).dot_product(&hit.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = match cannot_refract {
            true => unit_direction.reflect(&hit.normal),
            false => unit_direction.refract(&hit.normal, refraction_ratio),
        };

        let scattered = Ray {origin: hit.point, direction};
        return Some((scattered, attenuation));
    }
}
