use std::fmt;

#[derive(Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
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
