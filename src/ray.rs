use crate::{point::{Point3, Vector3}, material::{Material}};

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}


impl HitRecord {
    pub fn new(point: Point3, normal: Vector3, t: f64, front_face: bool) -> Self {
        return Self {point, normal, t, front_face};
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Box<dyn Material>)>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

pub fn new_hittable_list () -> HittableList{
    HittableList { objects: vec![] }
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Box<dyn Material>)> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;
        for object in &self.objects {
            match object.hit(ray, t_min, closest_so_far) {
                Some((hit, material)) => {
                    closest_so_far = hit.t;
                    hit_record = Some((hit, material));
                }
                None => ()
            }
        };
        return hit_record
    }
}