use std::fmt;

use crate::{ray::{HitRecord, Ray}, color::Color, point::{random_unit_vector, random_vector_in_unit_sphere, Vector3}};

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
        let refracted = refract(&unit_direction, &hit.normal, refraction_ratio);

        let scattered = Ray {origin: hit.point, direction: refracted};
        return Some((scattered, attenuation));
    }
}

fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f64) -> Vector3 {
    let minus_uv = *uv * -1.0;
    let cos_theta = f64::min(minus_uv.dot_product(n), 1.0);
    let r_out_perp = (*uv + *n * cos_theta ) * etai_over_etat;
    let r_out_parallel = *n * (- f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())));
    return r_out_perp + r_out_parallel;
}

#[cfg(test)]
mod tests {
    use approx::relative_eq;

    use crate::{point::Vector3, material::refract};

    #[test]
    fn test_refract_passes_straight_throught_same_material() {
        let incoming = Vector3::new(1.0, 1.0, 0.0).unit_direction();
        let normal = Vector3::new(0.0, 0.0, 1.0);
        let refraction_ratio = 1.0;
        let result = refract(&incoming, &normal, refraction_ratio);
        let expected = Vector3::new(1.0, 1.0, 0.0).unit_direction();
        assert!(
            relative_eq!(result.x, expected.x, epsilon = 1.0e-6)
        );
        assert!(
            relative_eq!(result.y, expected.y, epsilon = 1.0e-6)
        );
        assert!(
            relative_eq!(result.z, expected.z, epsilon = 1.0e-6)
        );
    }
}