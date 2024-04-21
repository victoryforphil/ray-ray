use std::ops::{Add, Div, Mul, Neg, Sub};

const TUPLE_EPSILON: f64 = 0.00001;
#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    /// # Summary
    /// Creates a new tuple
    ///
    /// # Examples
    /// ```
    /// use ray_ray::math::Tuple;
    /// let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    /// # Summary
    /// Creates a new point tuple (w == 1.0)
    ///
    /// # Examples
    /// ```
    /// use ray_ray::math::Tuple;
    /// let tuple = Tuple::point(4.3, -4.2, 3.1);
    /// assert_eq!(tuple.w, 1.0);
    /// ```
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    /// # Summary
    /// Creates a new vector tuple (w == 0.0)
    ///
    /// # Examples
    /// ```
    /// use ray_ray::math::Tuple;
    /// let tuple = Tuple::vector(4.3, -4.2, 3.1);
    /// assert_eq!(tuple.w, 0.0);
    /// ```
    ///
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    /// # Summary
    ///  Returns true if the tuple is a point (w == 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use ray_ray::math::Tuple;
    /// let tuple = Tuple::point(4.3, -4.2, 3.1);
    /// assert_eq!(tuple.is_point(), true);
    /// ```
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    /// # Summary
    /// Returns true if the tuple is a vector (w == 0.0)
    /// Returns false if the tuple is a point (w == 1.0)
    ///
    /// # Examples
    /// ```
    /// use ray_ray::math::Tuple;
    /// let tuple = Tuple::vector(4.3, -4.2, 3.1);
    /// assert_eq!(tuple.is_vector(), true);
    /// ```
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl From<[f64; 4]> for Tuple {
    fn from(value: [f64; 4]) -> Self {
        Tuple {
            x: value[0],
            y: value[1],
            z: value[2],
            w: value[3],
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON
            && (self.y - other.y).abs() < TUPLE_EPSILON
            && (self.z - other.z).abs() < TUPLE_EPSILON
            && (self.w - other.w).abs() < TUPLE_EPSILON
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl Tuple {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let m = self.magnitude();
        Tuple {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }

    pub fn dot(&self, rhs: &Tuple) -> f64{
        (self.x * rhs.x) + 
        (self.y * rhs.y) + 
        (self.z * rhs.z) + 
        (self.w * rhs.z) 
    }

    pub fn cross(&self, rhs: &Tuple) -> Tuple{
        Tuple::vector(self.y * rhs.z - self.z * rhs.y, 
            self.z * rhs.x - self.x * rhs.z, 
            self.x * rhs.y - self.y * rhs.x)
    }
}


#[cfg(test)]
mod test {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_new_tuple_point() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 1.0);

        assert_eq!(tuple.is_point(), true);
        assert_eq!(tuple.is_vector(), false);
    }

    #[test]
    pub fn test_new_tuple_vector() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 0.0);

        assert_eq!(tuple.is_vector(), true);
        assert_eq!(tuple.is_point(), false);
    }

    #[test]
    pub fn test_new_vector() {
        let tuple = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 0.0);

        assert_eq!(tuple.is_vector(), true);
        assert_eq!(tuple.is_point(), false);
    }

    #[test]
    pub fn test_new_point() {
        let tuple = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 1.0);

        assert_eq!(tuple.is_point(), true);
        assert_eq!(tuple.is_vector(), false);
    }

    #[test]
    pub fn test_tuple_equality() {
        let tuple1 = Tuple::new(4.3, -4.2, 3.1, 1.0);
        let tuple2 = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(tuple1, tuple2);
    }

    #[test]
    pub fn test_tuple_addition() {
        let tuple1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let tuple2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let result = tuple1 + tuple2;
        assert_eq!(result, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    pub fn test_point_subtraction() {
        let point1 = Tuple::point(3.0, 2.0, 1.0);
        let point2 = Tuple::point(5.0, 6.0, 7.0);

        let ans = point1 - point2;
        assert_eq!(ans.is_vector(), true);
        assert_eq!(ans, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    pub fn test_point_subtraction_vector() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        let a = p - v;
        assert_eq!(a.is_point(), true);
        assert_eq!(a, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    pub fn test_vector_subtraction() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        let a = v1 - v2;
        assert_eq!(a.is_vector(), true);
        assert_eq!(a, Tuple::vector(-2.0, -4.0, -6.));
    }

    #[test]
    pub fn test_vector_subtract_zero_vector() {
        let vz = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        let a = vz - v;
        assert_eq!(a.is_vector(), true);
        assert_eq!(a, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    pub fn test_negate_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let n_a = -a;
        assert_eq!(n_a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    pub fn test_scalar_tuple_mul() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let a = t * 3.5;
        assert_eq!(a, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    pub fn test_fraction_tuple_mul() {
        let t: Tuple = [1.0, -2.0, 3.0, -4.0].into();
        let a = t * 0.5;
        assert_eq!(a, [0.5, -1.0, 1.5, -2.0].into())
    }
    #[test]
    pub fn test_fraction_tuple_div() {
        let t: Tuple = [1.0, -2.0, 3.0, -4.0].into();
        let a = t / 2.0;
        assert_eq!(a, [0.5, -1.0, 1.5, -2.0].into())
    }

    #[test]
    pub fn test_tuple_mangitude() {
        {
            let t = Tuple::vector(1.0, 0.0, 0.0);
            let m = t.magnitude();
            assert_eq!(m, 1.0);
        }

        {
            let t = Tuple::vector(0.0, 1.0, 0.0);
            let m = t.magnitude();
            assert_eq!(m, 1.0);
        }

        {
            let t = Tuple::vector(0.0, 0.0, 1.0);
            let m = t.magnitude();
            assert_eq!(m, 1.0);
        }

        {
            let t = Tuple::vector(1.0, 2.0, 3.0);
            let m = t.magnitude();
            assert_eq!(m, (14.0_f64).sqrt());
        }

        {
            let t = Tuple::vector(-1.0, -2.0, -3.0);
            let m = t.magnitude();
            assert_eq!(m, (14.0_f64).sqrt());
        }
    }

    #[test]
    pub fn test_tuple_normalize() {
        {
            let t = Tuple::vector(4.0, 0.0, 0.0);
            let t_n = t.normalize();
            assert_eq!(t_n, Tuple::vector(1.0, 0.0, 0.0))
        }

        {
            let t = Tuple::vector(1.0, 2.0, 3.0);
            let t_n = t.normalize();
            assert_eq!(
                t_n,
                Tuple::vector(
                    1.0 / (14.0_f64).sqrt(),
                    2.0 / (14.0_f64).sqrt(),
                    3.0 / (14.0_f64).sqrt()
                )
            )
        }

        {
            let t = Tuple::vector(4.0, 0.0, 0.0);
            let t_n = t.normalize();
            assert_eq!(
                t_n.magnitude(),
                1.0,
                "Normalize should have a magnitude of 1"
            );
        }
    }

    #[test]
    pub fn test_tuple_dot(){
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let dot = a.dot(&b);
        assert_eq!(dot, 20.0);
    }

    #[test]
    pub fn test_tuple_cross(){
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let cross_ab = a.cross(&b);
        let corss_ba = b.cross(&a);
        assert_eq!(cross_ab, Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(corss_ba, Tuple::vector(1.0, -2.0, 1.0));
    }
}
