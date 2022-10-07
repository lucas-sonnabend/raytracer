


use std::{ops::{Add, Sub, Mul, Div}};

use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z 
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared() as f64)
    }

    pub fn unit_direction(&self) -> Vector3 {
        let len = self.length();
        *self / len
    }

    pub fn dot_product(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        return (self.x.abs() < eps) && (self.y.abs() < eps) && (self.z.abs() < eps);
    }

    pub fn reflect(&self, n: &Vector3) -> Vector3 {
        return *self - (*n * self.dot_product(n) * 2.0);
    }
}


impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, _rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, t: f64) -> Vector3 {
        Vector3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, t: f64) -> Vector3 {
        Vector3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

pub fn random_vector_in_unit_sphere() -> Vector3 {
    let mut rng = rand::thread_rng();
    loop {
        let random_vec = Vector3 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            z: rng.gen_range(-1.0..1.0),
        };
        if random_vec.length_squared() < 1.0 {
            return random_vec;
        }
    }
}

pub fn random_unit_vector() -> Vector3 {
    random_vector_in_unit_sphere().unit_direction()
}

pub type Point3 = Vector3;

#[cfg(test)]
mod tests {
    use crate::point::Vector3;

    #[test]
    fn test_adding_to_positive_points() {
        let point1 = Vector3 {x:1.0, y:2.0, z:3.0};
        let point2 = Vector3 {x: 10.0, y:20.0, z:30.0};
        let result = point1 + point2;
        assert_eq!(result, Vector3 {x:11.0, y: 22.0, z: 33.0});
    }
    
    #[test]
    fn test_length_squared() {
        let point1 = Vector3 {x:3.0, y:4.0, z:5.0};
        assert_eq!(point1.length_squared(), 50.0);
    }
    
    #[test]
    fn test_length() {
        let point1 = Vector3 {x:3.0, y:4.0, z:12.0};
        assert_eq!(point1.length(), 13.0);
    }
    
    #[test]
    fn test_multiply_by_scalar() {
        let point1 = Vector3 {x:3.0, y:4.0, z:5.0};
        let expected = Vector3 {x:6.0, y:8.0, z:10.0};
        assert_eq!(point1 * 2.0, expected);
    }
    
    #[test]
    fn test_dot_product() {
        let point1 = Vector3 {x:2.0, y:3.0, z:5.0};
        let point2 = Vector3 {x:100.0, y:10.0, z:1.0};
        assert_eq!(point1.dot_product(&point2), 235.0);
    }

    #[test]
    fn test_reflect() {
        let vector = Vector3 {x: 1.0, y: -1.0, z: 1.0};
        let norm = Vector3 {x: 0.0, y: 1.0, z: 0.0};
        let expected = Vector3 {x: 1.0, y: 1.0, z: 1.0};
        assert_eq!(vector.reflect(&norm), expected);
    }
}
