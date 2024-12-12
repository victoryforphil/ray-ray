use std::{fmt::{Debug, Display}, ops::{Add, Div, Mul, Neg, Sub}};


#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
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
    

    /// Calculates the Magnitude
    pub fn magnitude(&self) -> f64 {
        let sum = self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.) + self.w.powf(2.);
        sum.sqrt()
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        self.x = self.x / mag;
        self.y = self.y / mag;
        self.z = self.z / mag;
        self.w = self.w / mag;
    }

    pub fn normalized(self) -> Self {
        let mut new = self.clone();
        new.normalize();
        new
    }

    pub fn dot(&self, rhs: &Tuple) -> f64{
        return 
        (self.x * rhs.x) +
        (self.y * rhs.y) +
        (self.z * rhs.z) +
        (self.w * rhs.w) 
    }

    /// Calculates cross product
    /// Returns a vector / tuple
    pub fn cross(&self, rhs: &Tuple) -> Tuple{
        let x = (self.y * rhs.z) - (self.z * rhs.y);
        let y = (self.z * rhs.x) - (self.x * rhs.z);
        let z = (self.x * rhs.y) - (self.y * rhs.x);
        Self::new_vector(x, y, z)
    }
    
}

impl Display for Tuple{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tuple({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}
impl Debug for Tuple{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tuple({}, {}, {}, {})", self.x, self.y, self.z, self.w)
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

    #[test]
    pub fn test_mangitude(){
        let v1:Tuple = [1., 0., 0.].into();
        assert_eq!(v1.magnitude(), 1.0);

        let v2:Tuple = [0., 1., 0.].into();
        assert_eq!(v2.magnitude(), 1.0);

        let v3:Tuple = [0., 0., 1.].into();
        assert_eq!(v3.magnitude(), 1.0);

        let v4:Tuple = [1., 2., 3.].into();
        let a4 = (14.0_f64).sqrt();
        assert_eq!(v4.magnitude(), a4);

        let v5:Tuple = [-1.,- 2., -3.].into();
        let a5 = (14.0_f64).sqrt();
        assert_eq!(v5.magnitude(), a5);
    }

    #[test]
    pub fn test_normalize(){

        // Verify normalize / normalized to the same thing

        let mut v_n:Tuple = [4., 3., 2.].into();
        let v_m = v_n.normalized();

        v_n.normalize();

        assert_eq!(v_m, v_n);


        let v1: Tuple = [4., 0., 0.].into();
        assert_eq!(v1.normalized(), [1.0, 0., 0.].into());

        let v2: Tuple = [1., 2., 3.].into();
        assert_eq!(v2.normalized(), [
            (1. / 14_f64.sqrt()),
            (2. / 14_f64.sqrt()),
            (3. / 14_f64.sqrt())
        ].into());

        assert_eq!(v1.normalized().magnitude(), 1.0);
        assert_eq!(v2.normalized().magnitude(), 1.0);
    }

    #[test]
    pub fn test_dot_product(){

        let v_a:Tuple = [1., 2., 3.].into();
        let v_b:Tuple = [2., 3., 4.].into();

        let a = v_a.dot(&v_b);
        assert_eq!(a, 20.0);
    }

    #[test]
    pub fn test_cross_product(){
        let v_a: Tuple = [1., 2., 3.].into();
        let v_b: Tuple = [2., 3., 4.].into();
        let a_b = v_a.cross(&v_b);
        let b_a = v_b.cross(&v_a);
        assert_eq!(a_b, [-1., 2., -1.].into());
        assert_eq!(b_a, [1., -2., 1.].into());
    }
}