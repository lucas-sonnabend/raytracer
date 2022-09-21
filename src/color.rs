use std::fmt;
use std::ops::{Add, Mul, Div};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn gamma_correct(&self) -> Color {
        // correct by a gamma=2.0, which means taking it to the power of 1/2.0
        Color {
            r: f64::sqrt(self.r),
            g: f64::sqrt(self.g),
            b: f64::sqrt(self.b),
        }
    }
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

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, t: f64) -> Color {
        Color {
            r: self.r / t,
            g: self.g / t,
            b: self.b / t
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
