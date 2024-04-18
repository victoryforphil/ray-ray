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

#[cfg(test)]
mod test {
    use super::*;

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
}
