use crate::{point::{Point3, Vector3, random_in_unit_disk}, ray::Ray};

pub struct Camera {
    origin: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Vector3,
    u: Vector3,
    v: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vector3,
        vfov: f64, // vertical field of view
        aspect_ratio: f64,
        aperture: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_direction();
        let u = vup.cross_product(&w).unit_direction();
        let v = w.cross_product(&u);

        let focus_dist = (look_from - look_at).length();
        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u, v,
            lens_radius: aperture / 2.0,
        }
    }
    
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        return Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset,
        };
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}