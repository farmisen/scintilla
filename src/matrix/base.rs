use crate::tuple::Tuple;
use std::ops::{Index, IndexMut, Mul};
use approx::{AbsDiffEq};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Matrix {
    pub(in crate::matrix) data: Vec<f64>,
    pub(in crate::matrix) width: usize,
    pub(in crate::matrix) height: usize,
}

impl Matrix {
    pub fn zeroed(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height],
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut res = Matrix::zeroed(size, size);
        for i in 0..size {
            res[(i, i)] = 1.;
        }
        res
    }

    pub fn from_rows(rows: Vec<Vec<f64>>) -> Self {
        let h = rows.len();
        let w = if h == 0 { 0 } else { rows[0].len()};
        Self {
            width:w,
            height:h,
            data: rows.into_iter().flat_map(|x| x.into_iter()).collect(),
        }
    }

    pub fn from_tuple(tuple: Tuple) -> Self {
        Self {
            width: 1,
            height: 4,
            data: vec![tuple.x, tuple.y, tuple.z, tuple.w],
        }
    }

    
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    // (row, col)
    fn index(&self, indice: (usize, usize)) -> &Self::Output {
        &self.data[indice.0 * self.width + indice.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    // (row, col)
    fn index_mut(&mut self, indice: (usize, usize)) -> &mut Self::Output {
        &mut self.data[indice.0 * self.width + indice.1]
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        let mut data = Vec::new();
        for row in 0..self.height {
            for col in 0..other.width {
                let mut value = 0.0;
                for i in 0..self.width {
                    value += self[(row, i)] * other[(i, col)]
                }

                data.push(value)
            }
        }

        Matrix {
            width: other.width,
            height: self.height,
            data,
        }
    }
}

impl Mul<Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let other = Matrix::from_tuple(other);
        let res = self * &other;
        Tuple::new(res[(0, 0)], res[(1, 0)], res[(2, 0)], res[(3, 0)])
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        for row in 0..self.height {
            res.push_str("| ");
            for col in 0..self.width {
                res.push_str(&format!("{} ", self[(row, col)]));
            }
            res.push_str("|\n");
        }
        write!(f, "{}", res)
    }
}

impl AbsDiffEq for Matrix {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1.0E-14f64
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        self.width == other.width
            && self.height == other.height
            && self
                .data
                .iter()
                .zip(other.data.iter())
                .fold(true, |accu, (a, b)| {
                    accu && f64::abs_diff_eq(&a, &b, epsilon)
                })
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;
    use crate::tuple::Tuple;

    #[test]
    fn it_creates_2_by_2_matrices() {
        let matrix = Matrix::from_rows(vec![vec![-3., 5.], vec![1., -2.]]);
        assert_abs_diff_eq!(matrix[(0, 0)], -3.);
        assert_abs_diff_eq!(matrix[(0, 1)], 5.);
        assert_abs_diff_eq!(matrix[(1, 0)], 1.);
        assert_abs_diff_eq!(matrix[(1, 1)], -2.);
    }

    #[test]
    fn it_creates_3_by_3_matrices() {
        let matrix = Matrix::from_rows(
            vec![vec![-3., 5., 0.], vec![1., -2., 7.], vec![0., 1., 1.]],
        );
        assert_abs_diff_eq!(matrix[(0, 0)], -3.);
        assert_abs_diff_eq!(matrix[(1, 1)], -2.);
        assert_abs_diff_eq!(matrix[(2, 2)], 1.);
    }

    #[test]
    fn it_creates_4_by_4_matrices() {
        let matrix = Matrix::from_rows(
            vec![
                vec![1., 2., 3., 4.],
                vec![5.5, 6.5, 7.5, 8.5],
                vec![9., 10., 11., 1.],
                vec![13.5, 14.5, 15.5, 16.5],
            ],
        );
        assert_abs_diff_eq!(matrix[(0, 0)], 1.);
        assert_abs_diff_eq!(matrix[(0, 3)], 4.);
        assert_abs_diff_eq!(matrix[(1, 0)], 5.5);
        assert_abs_diff_eq!(matrix[(1, 2)], 7.5);
        assert_abs_diff_eq!(matrix[(2, 2)], 11.);
        assert_abs_diff_eq!(matrix[(3, 0)], 13.5);
        assert_abs_diff_eq!(matrix[(3, 2)], 15.5);
    }

    #[test]
    fn it_asserts_as_equal_identical_matrices() {
        let matrix1 = Matrix::from_rows(
            vec![
                vec![1., 2., 3., 4.],
                vec![5., 6., 7., 8.],
                vec![9., 8., 7., 6.],
                vec![5., 4., 3., 2.],
            ],
        );
        let matrix2 = Matrix::from_rows(
            vec![
                vec![1., 2., 3., 4.],
                vec![5., 6., 7., 8.],
                vec![9., 8., 7., 6.],
                vec![5., 4., 3., 2.],
            ],
        );
        assert_abs_diff_eq!(matrix1, matrix2);
    }

    #[test]
    fn it_asserts_as_not_equal_different_matrices() {
        let matrix1 = Matrix::from_rows(
            vec![
                vec![1., 2., 3., 4.],
                vec![5., 6., 7., 8.],
                vec![9., 8., 7., 6.],
                vec![5., 4., 3., 2.],
            ],
        );
        let matrix2 = Matrix::from_rows(
            vec![
                vec![0., 2., 3., 4.],
                vec![5., 0., 7., 8.],
                vec![9., 8., 0., 6.],
                vec![5., 4., 3., 0.],
            ],
        );
        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn it_multiply_matrices() {
        let matrix1 = Matrix::from_rows(
            vec![
                vec![1., 2., 3., 4.],
                vec![5., 6., 7., 8.],
                vec![9., 8., 7., 6.],
                vec![5., 4., 3., 2.],
            ],
        );
        let matrix2 = Matrix::from_rows(
            vec![
                vec![-2., 1., 2., 3.],
                vec![3., 2., 1., -1.],
                vec![4., 3., 6., 5.],
                vec![1., 2., 7., 8.],
            ],
        );
        let expected = Matrix::from_rows(
            vec![
                vec![20., 22., 50., 48.],
                vec![44., 54., 114., 108.],
                vec![40., 58., 110., 102.],
                vec![16., 26., 46., 42.],
            ],
        );
        assert_abs_diff_eq!(&matrix1 * &matrix2, expected);
    }

    #[test]
    fn it_multiply_matrices_and_tuples() {
        let matrix = Matrix::from_rows(
            vec![
                vec![1., 2., 3., 4.],
                vec![2., 4., 4., 2.],
                vec![8., 6., 4., 1.],
                vec![0., 0., 0., 1.],
            ],
        );
        let tuple = Tuple::new(1., 2., 3., 1.);
        let expected = Tuple::new(18., 24., 33., 1.);
        assert_abs_diff_eq!(&matrix * tuple, expected);
    }

    #[test]
    fn it_can_be_multiplied_by_the_identity_matrix() {
        let matrix = Matrix::from_rows(
            vec![
                vec![0., 1., 2., 4.],
                vec![1., 2., 4., 8.],
                vec![2., 4., 8., 16.],
                vec![4., 8., 16., 32.],
            ],
        );
        let identity = Matrix::identity(4);
        assert_abs_diff_eq!(&matrix * &identity, matrix);
    }
}
