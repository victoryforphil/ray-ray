use super::Tuple;

pub struct Point{
    pub t: Tuple,
}

impl Default for Point{
    fn default() -> Self{
        Point{
            t: Tuple::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Point{
            t: Tuple::new(x, y, z, 1.0),
        }
    }
}
impl From<Tuple> for Point{
    fn from(t: Tuple) -> Self{
        if !t.is_point(){
            panic!("Tuple is not a point: {:?}", t);
        }
        Point{
            t,
        }
    }
}

impl From<Point> for Tuple{
    fn from(p: Point) -> Self{
        p.t
    }
}
#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_point_new() {
        let p = Point::new(4.3, -4.2, 3.1);
        assert_eq!(p.t.x, 4.3);
        assert_eq!(p.t.y, -4.2);
        assert_eq!(p.t.z, 3.1);
        assert_eq!(p.t.w, 1.0);
    }

    #[test]
    fn test_point_tuple() {
        let t = Tuple::new(4.3, -4.2, 3.1, 1.0);
        let p = Point::from(t);
        assert_eq!(p.t.x, 4.3);
        assert_eq!(p.t.y, -4.2);
        assert_eq!(p.t.z, 3.1);
        assert_eq!(p.t.w, 1.0);

        let p = Point::new(4.3, -4.2, 3.1);

        let t = Tuple::from(p);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
    }
}