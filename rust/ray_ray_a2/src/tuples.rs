use std::ops::{Add, Div, Mul, Neg, Sub};


#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Tuple{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self{
        Self {
            x,y,z,w
        }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self{
        Self { x: x, y: y, z: z, w: 1.0 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self{
        Self { x: x, y: y, z: z, w: 0.0 }
    }
    
    /// Returns if the current tuple is a point
    /// (if w = 1.0)
    pub fn is_point(&self) -> bool{
        self.w == 1.0
    }

    /// Returns if the current tuple is a vector
    /// (if w = 0.0)
    pub fn is_vector(&self) -> bool{
        self.w == 0.0
    }
}


impl From<[f64;4]> for Tuple{
    fn from(value: [f64;4]) -> Self {
        Self { x: value[0], y: value[1], z: value[2], w: value[3] }
    }
}

impl From<(f64, f64, f64,f64)> for Tuple{
    fn from(value: (f64, f64, f64,f64)) -> Self {
         let (x, y, z, w) = value;
         Self { x: x, y: y, z: z, w: w }
    }
}

impl From<[f64;3]> for Tuple{
    fn from(value: [f64;3]) -> Self {
        Self::new_vector(value[0], value[1], value[2])
    }
}

impl From<(f64, f64, f64)> for Tuple{
    fn from(value: (f64, f64, f64)) -> Self {
         let (x, y, z) = value;
         Self::new_point(x, y, z)
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Self{
            x: 0.0 - self.x,
            y: 0.0 - self.y,
            z: 0.0 - self.z,
            w: 0.0 - self.w,
        }
    }
}

impl Add for Tuple{
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl Sub for Tuple{
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}


impl Mul<f64> for Tuple{
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }

}

impl Div<f64> for Tuple{
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Self{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }

}

#[cfg(test)]

mod test{
    
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    pub fn test_construction(){
        let t1 = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(t1.x, 4.3);
        assert_eq!(t1.y, -4.2);
        assert_eq!(t1.z, 3.1);
        assert_eq!(t1.w, 1.0);

        let array = [4.3, -4.2,3.1, 1.0];
        let t_array: Tuple = array.into();

        assert_eq!(t1, t_array);

        let tuple = (4.3, -4.2, 3.1, 1.0);
        let t_tuple: Tuple = tuple.into();

        assert_eq!(t1, t_tuple);
    }

    #[test]
    pub fn test_vector(){
        let t_vector = Tuple::new_vector(4.0, -4.0, 3.0);
        assert_eq!(t_vector, [4.0, -4.0, 3.0, 0.0].into());
        assert!(t_vector.is_vector());
        assert!(!t_vector.is_point());

        let t_auto: Tuple = [4.0, -4.0, 3.0].into();
        assert_eq!(t_auto, t_vector);
    }

    #[test]
    pub fn test_point(){
        let t_point = Tuple::new_point(4.0, -4.0, 3.0);
        assert_eq!(t_point, [4.0, -4.0, 3.0, 1.0].into());
        assert!(t_point.is_point());
        assert!(!t_point.is_vector());


        let t_auto: Tuple = (4.0, -4.0, 3.0).into();
        assert_eq!(t_auto, t_point);
    }

    #[test]
    pub fn test_add(){
        let t_1: Tuple = [3.0, -2.0, 5.0, 1.0].into();
        let t_2: Tuple = [-2.0, 3.0, 1.0, 0.0].into();

        let t_a = t_1 + t_2;

        assert_eq!(t_a, [1.0, 1.0, 6.0, 1.0].into());
    }

    #[test]
    pub fn test_sub_points(){
        let p1: Tuple = (3.0, 2.0, 1.0).into();
        let p2: Tuple = (5.0, 6.0, 7.0).into();

        let va = p1-p2;

        assert!(va.is_vector()); // Result is a vector! 
        assert_eq!(va, [-2.0, -4.0, -6.0].into());
    }

    #[test]
    pub fn test_sub_point_vector(){
        let p: Tuple = (3.0, 2.0, 1.0).into();
        let v: Tuple = [5.0, 6.0, 7.0].into();

        let pa = p-v;

        assert!(pa.is_point()); // Result is a point! 
        assert_eq!(pa, (-2.0, -4.0, -6.0).into());
    }

    #[test]
    pub fn test_sub_vector(){
        let v1: Tuple = [3.0, 2.0, 1.0].into();
        let v2: Tuple = [5.0, 6.0, 7.0].into();

        let va = v1-v2;

        assert!(va.is_vector()); // Result is a point! 
        assert_eq!(va, [-2.0, -4.0, -6.0].into());
    }

    #[test]
    pub fn test_sub_zero(){
        let v1: Tuple = [0.0, 0.0, 0.0].into();
        let v2: Tuple = [3.0, 2.0, 1.0].into();

        let va = v1-v2;

        assert!(va.is_vector()); // Result is a point! 
        assert_eq!(va, [-3.0, -2.0, -1.0].into());
    }

    #[test]
    pub fn test_negate(){
  
        let v: Tuple = [3.0, 2.0, 1.0].into();

        let va = -v;

        assert!(va.is_vector()); // Result is a point! 
        assert_eq!(va, [-3.0, -2.0, -1.0].into());
    }


    #[test]
    pub fn test_mul_scalar(){
  
        let t: Tuple = [1.0, -2.0, 3.0, -4.0].into();

        let ta = t * 3.5;

        assert_eq!(ta, [3.5, -7.0, 10.5, -14.0].into());
    }


    #[test]
    pub fn test_mul_fraction(){
  
        let t: Tuple = [1.0, -2.0, 3.0, -4.0].into();

        let ta = t * 0.5;

        assert_eq!(ta, [0.5, -1.0, 1.5, -2.0].into());
    }

    #[test]
    pub fn test_div_scalar(){
  
        let t: Tuple = [1.0, -2.0, 3.0, -4.0].into();

        let ta = t  / 2.0;

        assert_eq!(ta, [0.5, -1.0, 1.5, -2.0].into());
    }
}