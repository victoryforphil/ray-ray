#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Default for Tuple{
    fn default() -> Self{
        Tuple{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self{
        Tuple{
            x,
            y,
            z,
            w,
        }
    }

    pub fn is_point(&self) -> bool{
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool{
        self.w == 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_new() {
        let t = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 1.0);
    }

    #[test]
    fn test_tuple_is_point() {
        let t = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(t.is_point(), true);
    }

    #[test]
    fn test_tuple_is_vector() {
        let t = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(t.is_vector(), true);
    }
}