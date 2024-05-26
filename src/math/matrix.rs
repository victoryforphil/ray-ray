use std::fmt::Debug;

use log::debug;

use crate::math::tuple::Tuple;
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
    pub w: usize,
    pub h: usize,
}

impl Matrix {
    pub fn new_vec(data: Vec<Vec<f64>>) -> Self {
        let w = data.len();
        let h = data[0].len();
        Self { data, w: w, h: h }
    }

    pub fn new(data: &[&[f64]]) -> Self {
        let mut vecs = vec![vec![]];
        for n_row in 0..data.len() {
            let mut row = vec![];
            for n_col in 0..data[n_row].len() {
                row.push(data[n_row][n_col]);
            }
            vecs.push(row);
        }
        Self {
            data: vecs,
            w: data.len(),
            h: data[0].len(),
        }
    }
    /// # Summary
    /// Returns the size of the matrix (w, h)
    /// # Examples
    /// ```
    /// use ray_ray::math::Matrix;
    /// let m = Matrix::new(&[
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ]);
    /// assert_eq!(m.size(), (2, 2));
    /// ```
    pub fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    pub fn zero(size: (usize, usize)) -> Self {
        let mut data = Vec::with_capacity(size.0);
        for _ in 0..size.0 {
            let mut row = Vec::with_capacity(size.1);
            for _ in 0..size.1 {
                row.push(0.0);
            }
            data.push(row);
        }
        Self {
            data,
            w: size.0,
            h: size.1,
        }
    }

    pub fn indentity(size: (usize, usize)) -> Self {
        let mut m = Self::zero(size);
        for i in 0..m.w {
            m[i][i] = 1.0;
        }
        m
    }

    pub fn transpose(self) -> Self {
        let mut m = Matrix::zero((self.w, self.h));
        for a_row in 0..4 {
            for b_col in 0..4 {
                m[a_row][b_col] = self[b_col][a_row]
            }
        }
        m
    }
    // Returns a matrix 1 row and 1 col smaller
    pub fn sub_matrix(self) -> Matrix {
        let sw = self.w - 1;
        let sh = self.h - 1;
        let mut sub_matrix = Matrix::zero((sw, sh));

        for w in 0..sw {
            for h in 0..sh {
                sub_matrix[w][h] = self.data[w][h];
            }
        }

        sub_matrix
    }
    pub fn determinate(&self) -> f64 {
        // Only 2x2 matrix
        assert_eq!(self.w, 2);
        assert_eq!(self.h, 2);
        let a = self[0][0];
        let b = self[0][1];
        let c = self[1][0];
        let d = self[1][1];

        (a * d) - (b * c)
    }
}

/// Convert from &[[f64]] to Matrix of any size

impl From<&[&[f64]]> for Matrix {
    fn from(value: &[&[f64]]) -> Self {
        Self::new(value)
    }
}

impl From<Vec<Vec<f64>>> for Matrix {
    fn from(value: Vec<Vec<f64>>) -> Self {
        Self::new_vec(value)
    }
}

impl<const N: usize> From<&[&[f64; N]; N]> for Matrix {
    fn from(array: &[&[f64; N]; N]) -> Self {
        let mut data = Vec::with_capacity(N);
        for inner_array in array {
            debug!("Adding row to new vec{:?}", inner_array.to_vec());
            data.push(inner_array.to_vec());
        }
        Matrix { data, w: N, h: N }
    }
}

impl<const N: usize, const M: usize> From<[[f64; N]; M]> for Matrix {
    fn from(array: [[f64; N]; M]) -> Self {
        let data = array
            .iter()
            .map(|&inner_array| inner_array.to_vec())
            .collect();
        Matrix { data, w: N, h: M }
    }
}

/// Implement direct [][] access to the matrix data
impl std::ops::Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
/// Add (W,H) Index
impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Self::Output {
        // Only for 4.4 matrices
        assert_eq!(self.w, 4);
        assert_eq!(self.h, 4);
        let a = &self;
        let b = &rhs;
        let mut m = Matrix::zero((self.w, self.h));
        for a_row in 0..4 {
            for b_col in 0..4 {
                m[a_row][b_col] = (a[a_row][0] * b[0][b_col])
                    + (a[a_row][1] * b[1][b_col])
                    + (a[a_row][2] * b[2][b_col])
                    + (a[a_row][3] * b[3][b_col]);
            }
        }
        m
    }
}

impl std::ops::Mul<Matrix> for Tuple {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Tuple {
        // Only for 4.4 matrices
        assert_eq!(rhs.w, 4);
        assert_eq!(rhs.h, 4);
        let b: &Tuple = &self;
        let a: &Matrix = &rhs;
        let mut t = Tuple::new(0.0, 0.0, 0.0, 0.0);
        for a_row in 0..4 {
            t[a_row] = (a[a_row][0] * b[0])
                + (a[a_row][1] * b[1])
                + (a[a_row][2] * b[2])
                + (a[a_row][3] * b[3]);
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_matrix_4_4() {
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);

        assert_eq!(
            m,
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]
            ]
            .into()
        );

        assert_eq!(
            m.data,
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.5, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5]
            ]
        );
    }

    #[test]
    pub fn test_matrix_2_2() {
        let m = Matrix::from([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    pub fn test_matrix_3_3() {
        env_logger::init();
        let m = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    #[test]
    pub fn test_matrix_eq() {
        let m1 = Matrix::from(&[
            &[1.0, 2.0, 3.0, 4.0],
            &[5.0, 6.0, 7.0, 8.0],
            &[9.0, 8.0, 7.0, 6.0],
            &[5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix::from(&[
            &[1.0, 2.0, 3.0, 4.0],
            &[5.0, 6.0, 7.0, 8.0],
            &[9.0, 8.0, 7.0, 6.0],
            &[5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(m1 == m2, true);
    }

    #[test]
    pub fn test_matrix_neq() {
        let m1 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 4.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(m1 != m2, true);
    }

    #[test]
    pub fn test_matrix_mul_4() {
        let a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let b = Matrix::from(&[
            &[-2.0, 1.0, 2.0, 3.0],
            &[3.0, 2.0, 1.0, -1.0],
            &[4.0, 3.0, 6.0, 5.0],
            &[1.0, 2.0, 7.0, 8.0],
        ]);

        let p = a * b;

        assert_eq!(
            p,
            (&[
                &[20.0, 22.0, 50.0, 48.0],
                &[44.0, 54.0, 114.0, 108.0],
                &[40.0, 58.0, 110.0, 102.0],
                &[16.0, 26.0, 46.0, 42.0]
            ])
                .into()
        );
    }

    #[test]
    pub fn test_matrix_mul_tuple() {
        let a = Matrix::from(&[
            &[1.0, 2.0, 3.0, 4.0],
            &[2.0, 4.0, 4.0, 2.0],
            &[8.0, 6.0, 4.0, 1.0],
            &[0.0, 0.0, 0.0, 1.0],
        ]);

        let b: Tuple = [1.0, 2.0, 3.0, 1.0].into();
        let p = b * a;
        assert_eq!(p, [18.0, 24.0, 33.0, 1.0].into());
    }

    #[test]
    pub fn test_matrix_identity() {
        let a = Matrix::from(&[
            &[0.0, 1.0, 2.0, 4.0],
            &[1.0, 2.0, 4.0, 8.0],
            &[2.0, 4.0, 8.0, 16.0],
            &[4.0, 8.0, 16.0, 32.0],
        ]);

        let b = Matrix::indentity(a.size());
        let c = a.clone() * b;
        assert_eq!(c, a); // A * I = A
    }

    #[test]
    pub fn test_matrix_transpose() {
        let a = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        assert_eq!(
            a.transpose(),
            [
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0]
            ]
            .into()
        )
    }

    #[test]
    pub fn test_matrix_transpose_identity() {
        let i: Matrix = Matrix::indentity((4, 4));
        assert_eq!(i.transpose(), Matrix::indentity((4, 4)));
    }
    #[test]
    pub fn test_matrix_determinate() {
        let a = Matrix::from(&[&[1.0, 5.0], &[-3.0, 2.0]]);
        assert_eq!(a.determinate(), 17.0);
    }

    #[test]
    pub fn test_matrix_submatrix_3x3() {
        let a = Matrix::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let b: Matrix = a.sub_matrix();

        assert_eq!(b, [[1.0, 5.0], [-3.0, 2.0]].into());
    }

    #[test]
    pub fn test_matrix_submatrix_4x4() {
        let a = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let b: Matrix = a.sub_matrix();
        assert_eq!(
            b,
            [[0.0, 9.0, 3.0], [9.0, 8.0, 0.0], [1.0, 8.0, 5.0]].into()
        );
    }
}
