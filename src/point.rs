


use std::{ops::{Add, Sub}, fmt};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn length_squared(&self) -> i32 {
        self.x * self.x + self.y * self.y + self.z * self.z 
    }
    fn length(&self) -> f32 {
        f32::sqrt(self.length_squared() as f32)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, _rhs: Point) -> Point {
        Point {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z
        }
    }
}

#[test]
fn test_adding_to_positive_points() {
    let point1 = Point {x:1, y:2, z:3};
    let point2 = Point {x: 10, y:20, z:30};
    let result = point1 + point2;
    assert_eq!(result, Point {x:11, y: 22, z: 33});
}

#[test]
fn test_length_squared() {
    let point1 = Point {x:3, y:4, z:5};
    assert_eq!(point1.length_squared(), 50);
}

#[test]
fn test_length() {
    let point1 = Point {x:3, y:4, z:12};
    assert_eq!(point1.length(), 13.0);
}
