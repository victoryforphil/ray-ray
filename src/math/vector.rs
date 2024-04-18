use super::Tuple;

pub struct Vector{
    pub t: Tuple,
}

impl Default for Vector{
    fn default() -> Self{
        Vector{
            t: Tuple::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Vector{
            t: Tuple::new(x, y, z, 0.0),
        }
    }
}

impl From<Tuple> for Vector{
    fn from(t: Tuple) -> Self{
        if !t.is_vector(){
            panic!("Tuple is not a vector: {:?}", t);
        }
        Vector{
            t,
        }
    }
}

impl From<Vector> for Tuple{
    fn from(v: Vector) -> Self{
        v.t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_new() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.t.x, 4.3);
        assert_eq!(v.t.y, -4.2);
        assert_eq!(v.t.z, 3.1);
        assert_eq!(v.t.w, 0.0);
    }

    #[test]
    fn test_vector_tuple() {
        let t = Tuple::new(4.3, -4.2, 3.1, 0.0);
        let v = Vector::from(t);
        assert_eq!(v.t.x, 4.3);
        assert_eq!(v.t.y, -4.2);
        assert_eq!(v.t.z, 3.1);
        assert_eq!(v.t.w, 0.0);
    }
}