// aka a Donut
use crate::{point::{Point3, Vector3}, ray::{Ray, HitRecord, Hittable}};

use roots::find_roots_quartic;

use approx::relative_eq;

// formula (x^2 + y^2 + z^2 + A^2 - B^2)^2 = 4A^2 (x^2 + y^2)
// plugging in the ray with a formula of ray.origin + t * ray.direction

// a: distance from center of torus to the center of the "tube"
// b: radius of the "tube"

// This donut is oriented on the y = 0 plane, you currently cannot tilt it
pub struct Torus {
    pub center: Point3,
    pub a: f64, // aka R aka major radius
    pub b: f64, // aka S aka minor radius
}

impl Torus {
    fn build_hit_record(&self, origin: Point3, direction: Vector3, t: f64) -> HitRecord {
        let point = origin + direction * t;

        let g = 1.0 - (self.a / f64::sqrt(point.x * point.x + point.y * point.y));
        let outward_normal = match g.is_infinite() {
            true => {
                Vector3 {x: 0.0, y: 0.0, z: 1.0}
            },
            false => {
                Vector3 {x: point.x * g, y: point.y * g, z: point.z}.unit_direction()
            }
        };
        let front_face = direction.dot_product(&outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => outward_normal * (-1.0),
        };
        HitRecord {
            point: point + self.center,
            normal,
            t,
            front_face,
        }
    }
}

impl Hittable for Torus {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let vantage = ray.origin - self.center;
        let direction = ray.direction.unit_direction();
    
        let f = 4.0 * self.a * self.a;
        let j = direction.length_squared();
        let k = vantage.dot_product(&direction);
        let l = vantage.length_squared() - self.a*self.a - self.b*self.b;

        let g = f * direction.y * direction.y;
        let h = f * (self.b * self.b - vantage.y * vantage.y);
        let i = 4.0*self.a*self.a*direction.y*direction.y;

        let a4 = j*j;
        let a3 = 4.0 * j * k;
        let a2 = 2.0 * j * l + 4.0*k*k + g + i;
        let a1 = 4.0 * k * l - 8.0*self.a*self.a*vantage.y*direction.y;
        let a0 = l*l - h;


        let roots = find_roots_quartic(a4, a3, a2, a1, a0);
        let roots_as_vector = match roots {
            roots::Roots::No(_) => { vec![] },
            roots::Roots::One(root_vector) => { root_vector.to_vec()},
            roots::Roots::Two(root_vector) => { root_vector.to_vec()},
            roots::Roots::Three(root_vector) => { root_vector.to_vec()},
            roots::Roots::Four(root_vector) => { root_vector.to_vec()},
        };
        let mut closest = None;
        for root in roots_as_vector {
            if root > t_min && root < t_max {
                match closest {
                    None => {closest =  Some(self.build_hit_record(vantage, direction, root)) }
                    Some(closest_so_far) => {
                        if root < closest_so_far.t {
                            closest = Some(self.build_hit_record(vantage, direction, root))
                        }
                    }
                }
            }
        }
        return closest
    }
}

#[test]
fn test_hit_torus_in_center() {
    let torus = Torus {center: Point3 {x:1.0, y: 2.0, z: 10.0}, a: 1.0, b: 0.1};
    let ray = Ray {
        origin: Point3 {x: 1.0, y: 2.0, z: 0.0},
        direction: crate::point::Vector3 { x: 0.0, y: 0.0, z: 1.0 },
    };
    let hit = torus.hit(&ray, 0.0, 100.0);
    let expected = HitRecord {
        point: Point3 {x:1.0, y: 2.0, z: 8.9},
        normal: Point3 {x: 0.0, y: 0.0, z: -1.0},
        t: 8.9,
        front_face: true,
    };
    assert_almost_equal(hit, Some(expected));
}

#[test]
fn test_hit_central_torus_in_center() {
    let torus = Torus {center: Point3 {x:0.0, y: 0.0, z: 0.0}, a: 1.0, b: 0.1};
    let ray = Ray {
        origin: Point3 {x: 0.0, y: 0.0, z: -5.0},
        direction: crate::point::Vector3 { x: 0.0, y: 0.0, z: 1.0 },
    };
    let hit = torus.hit(&ray, -100.0, 100.0);
    let expected = HitRecord {
        point: Point3 {x:0.0, y: 0.0, z: -1.1},
        normal: Point3 {x: 0.0, y: 0.0, z: -1.0},
        t: 3.9,
        front_face: true,
    };
    assert_almost_equal(hit, Some(expected));
}

#[test]
fn test_ray_missing_torus() {
    let sphere = Torus {center: Point3 {x:1.0, y: 2.0, z: 10.0}, a: 1.0, b: 0.2};
    let ray = Ray {
        origin: Point3 {x: 1.0, y: 2.0, z: 0.0},
        direction: crate::point::Vector3 { x: 1.0, y: 1.0, z: 1.0 },
    };
    let hit = sphere.hit(&ray, 0.0, 100.0);
    assert_almost_equal(hit, None);
}

pub fn assert_almost_equal(val: Option<HitRecord>, other: Option<HitRecord>) {
    let is_almost_equal = match (val, other) {
        (None, None) => true,
        (Some(v1), Some(v2)) => {
            vector_relative_eq(v1.point, v2.point) &&
            vector_relative_eq(v1.normal, v2.normal) &&
            relative_eq!(v1.t, v2.t, epsilon = 1.0e-6)
        },
        (None, Some(_)) => false,
        (Some(_), None) => false, 
    };
    assert!(is_almost_equal)
}

fn vector_relative_eq(v1: Vector3, v2: Vector3) -> bool {
    relative_eq!(v1.x, v2.x, epsilon = 1.0e-6) &&
    relative_eq!(v1.y, v2.y, epsilon = 1.0e-6) &&
    relative_eq!(v1.z, v2.z, epsilon = 1.0e-6)
}
