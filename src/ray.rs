use crate::point::{Point3, Vector3};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

pub fn new_hittable_list () -> HittableList{
    HittableList { objects: vec![] }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;
        for object in &self.objects {
            match object.hit(ray, t_min, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.t;
                    hit_record = Some(hit);
                }
                None => ()
            }
        };
        return hit_record
    }
}