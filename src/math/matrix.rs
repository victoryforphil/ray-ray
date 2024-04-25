use std::fmt::Debug;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<const W: usize, const H: usize> {
    pub data: [[f64; W]; H],
}



impl<const W: usize, const H: usize>  Matrix<W, H> {
    pub fn new(data: [[f64; W]; H]) -> Self{
        Self{
            data
        }
    }

    pub fn zero() -> Self{
      
        Self{
            data : [[0.0; W]; H]
        }
    }
}

impl<const W: usize, const H: usize> From<[[f64; W];H]> for Matrix<W, H>{
    fn from(value: [[f64; W];H]) -> Self {
        Self::new(value)
    }
}
/// Implement direct [][] access to the matrix data
impl<const W: usize, const H: usize> std::ops::Index<usize> for Matrix<W, H> {
    type Output = [f64; W];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}


impl<const W: usize, const H: usize> std::ops::IndexMut<usize> for Matrix<W, H> {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
/// Add (W,H) Index
impl<const W: usize, const H: usize> std::ops::Index<(usize, usize)> for Matrix<W, H> {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl std::ops::Mul< Matrix<4, 4>> for Matrix<4, 4> {
    type Output = Self;
    
    fn mul(self, rhs: Matrix<4, 4>) -> Self::Output {
        let a = &self;
        let b = &rhs;
        let mut m = Matrix::zero();
        for a_row in 0..4{
            for b_col in 0..4{
                m[a_row][b_col] = 
                (a[a_row][0] * b[0][b_col]) +
                (a[a_row][1] * b[1][b_col]) +
                (a[a_row][2] * b[2][b_col]) +
                (a[a_row][3] * b[3][b_col]);
                
            }
        }
        m
    }

    
}

#[cfg(test)]
mod tests{
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_matrix_4_4() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5,15.5,16.5]
        ]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[(0,3)], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);

        assert_eq!(m, [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5,15.5,16.5]
        ].into());

        assert_eq!(m.data, [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5,15.5,16.5]
        ]);
    }

    #[test]
    pub fn test_matrix_2_2(){
        let m = Matrix::new([
            [-3.0, 5.0],
            [1.0, -2.0]
        ]);
        
        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    pub fn test_matrix_3_3(){
        let m = Matrix::new([
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]
        ]);
        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    pub fn test_matrix_eq(){
        let m1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
            
        ]);

        let m2 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
            
        ]);

        assert_eq!(m1 == m2, true);
    }

    #[test]
    pub fn test_matrix_neq(){
        let m1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
            
        ]);

        let m2 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 4.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
            
        ]);

        assert_eq!(m1 != m2, true);
    }

    #[test]
    pub fn test_matrix_mul_4(){
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
            
        ]);

        let b = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let p = a * b;

        assert_eq!(p, [
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0]
        ].into());

        
    }
}