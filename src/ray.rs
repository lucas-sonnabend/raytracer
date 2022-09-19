use crate::point::{Point3, Vector3};

struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    fn at(self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }
}