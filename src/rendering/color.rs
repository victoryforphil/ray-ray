use std::ops::{Add, Mul, Sub};
const COLOR_EPSILON: f64 = 0.000001;
#[derive(Debug, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0);
    }
}
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        (self.r - other.r).abs() < COLOR_EPSILON
            && (self.g - other.g).abs() < COLOR_EPSILON
            && (self.b - other.b).abs() < COLOR_EPSILON
    }
}
impl From<[f64; 3]> for Color {
    fn from(value: [f64; 3]) -> Self {
        Color {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_color_new() {
        let color = Color::new(-0.4, 0.4, 1.7);
        assert_eq!(color.r, -0.4);
        assert_eq!(color.g, 0.4);
        assert_eq!(color.b, 1.7)
    }

    #[test]
    pub fn test_color_from_array() {
        let color: Color = [-0.4, 0.4, 1.7].into();
        assert_eq!(color, Color::new(-0.4, 0.4, 1.7));
    }

    #[test]
    pub fn test_color_add() {
        let c1: Color = [0.9, 0.6, 0.75].into();
        let c2: Color = [0.7, 0.1, 0.25].into();
        let ca = c1 + c2;
        assert_eq!(ca, [1.6, 0.7, 1.0].into());
    }

    #[test]
    pub fn test_color_sub() {
        let c1: Color = [0.9_f64, 0.6, 0.75].into();
        let c2: Color = [0.7_f64, 0.1, 0.25].into();
        let cs = c1 - c2;
        assert_eq!(cs, [0.2, 0.5, 0.5].into());
    }

    #[test]
    pub fn test_color_mul_scalar() {
        let c: Color = [0.2, 0.3, 0.4].into();
        let cm = c * 2.0;
        assert_eq!(cm, [0.4, 0.6, 0.8].into());
    }

    #[test]
    pub fn test_color_mul() {
        let c1: Color = [1.0, 0.2, 0.4].into();
        let c2: Color = [0.9, 1.0, 0.1].into();
        let cm = c1 * c2;
        assert_eq!(cm, [0.9, 0.2, 0.04].into());
    }
}
