use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        Color {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, t: f64) -> Color {
        Color {
            r: self.r * t,
            g: self.g * t,
            b: self.b * t
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ir = (255.999 * self.r) as i32;
        let ig = (255.999 * self.g) as i32;
        let ib = (255.999 * self.b) as i32;
        write!(f, "{} {} {}", ir, ig, ib)
    }
}

#[test]
fn test_color_fmt_for_white() {
    let color = Color {r:1.0, g: 1.0, b: 1.0};
    assert_eq!(format!("{color}"), String::from("255 255 255"));
}
