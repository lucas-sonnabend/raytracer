use crate::{point::{Point3}, ray::{Ray, HitRecord, Hittable}, material::{LambertianMaterial}};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: LambertianMaterial,
}

impl Sphere {
    fn build_hit_record(&self, ray: &Ray, t: f64) -> HitRecord {
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let front_face = ray.direction.dot_product(&outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => outward_normal * (-1.0),
        };
        HitRecord {
            point,
            normal,
            material: &self.material,
            t,
            front_face,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot_product(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None
        } else {
            let sqrtd = f64::sqrt(discriminant);

            // find the nearest root that lies in the acceptable range
            let mut root =  ( - half_b - sqrtd) / a;
            if root < t_min || root > t_max {
                root = (-half_b + sqrtd) / a;
                if root < t_min || root > t_max {
                    return None
                }
            }
            return Some(self.build_hit_record(ray, root))
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{sphere::Sphere, point::Point3, ray::{Ray, HitRecord, Hittable}, color::Color, material::LambertianMaterial};

    #[test]
    fn test_hit_sphere_in_center() {
        let material = LambertianMaterial {
            albedo: Color {r: 127.0, g: 127.0, b: 127.0}
        };
        let sphere = Sphere {
            center: Point3 {x:1.0, y: 2.0, z: 10.0},
            radius: 1.0,
            material: material,
        };
        let ray = Ray {
            origin: Point3 {x: 1.0, y: 2.0, z: 0.0},
            direction: crate::point::Vector3 { x: 0.0, y: 0.0, z: 1.0 },
        };
        let hit = sphere.hit(&ray, 0.0, 100.0);
        let expected = HitRecord {
            point: Point3 {x:1.0, y: 2.0, z: 9.0},
            normal: Point3 {x: 0.0, y: 0.0, z: -1.0},
            material: &material,
            t: 9.0,
            front_face: true,
        };
        assert_eq!(
            hit,
            Some(expected),
            "actual hit {:?} was different from expected hit {:?}",
            hit,
            Some(expected)
        );
    }

    #[test]
    fn test_ray_missing_sphere() {
         let material = LambertianMaterial {
            albedo: Color {r: 127.0, g: 127.0, b: 127.0}
        };
        let sphere = Sphere {
            center: Point3 {x:1.0, y: 2.0, z: 10.0},
            radius: 1.0,
            material: material,
        };
        let ray = Ray {
            origin: Point3 {x: 1.0, y: 2.0, z: 0.0},
            direction: crate::point::Vector3 { x: 1.0, y: 1.0, z: 1.0 },
        };
        let hit = sphere.hit(&ray, 0.0, 100.0);
        assert_eq!(hit, None);
    }
}


