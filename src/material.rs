use crate::{ray::{HitRecord, Ray}, color::Color, point::{random_unit_vector}};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LambertianMaterial {
    pub albedo: Color,
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let mut rng = rand::thread_rng();
        let scatter_dir = hit.normal + random_unit_vector(&mut rng);
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
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray_in.direction.unit_direction().reflect(&hit.normal);
        let scattered = Ray {origin: hit.point, direction: reflected};
        if scattered.direction.dot_product(&hit.normal) > 0.0 {
            return Some((scattered, self.albedo));
        } else {
            return None;
        }
    }
}
