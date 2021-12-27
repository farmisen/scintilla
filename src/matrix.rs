// use approx::{AbsDiffEq, RelativeEq};
use crate::tuple::Tuple;
use approx::{AbsDiffEq};
use std::fmt;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, PartialEq)]
pub struct Matrix {
    data: Vec<f64>,
    width: usize,
    height: usize,
}

// pub struct Indice (usize, usize);

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

    pub fn from_rows(width: usize, height: usize, rows: Vec<Vec<f64>>) -> Self {
        Self {
            width,
            height,
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

    pub fn transposed(&self) -> Self {
        let mut res = Matrix::zeroed(self.height, self.width);
        for row in 0..res.height {
            for col in 0..res.width {
                res[(row, col)] = self[(col, row)];
            }
        }
        res
    }

    pub fn det(&self) -> f64 {
        if self.width == 2 && self.height == 2 {
            self[(0, 0)] * self[(1, 1)] - self[(1, 0)] * self[(0, 1)]
        } else {
            let mut res = 0.;
            for c in 0..self.width {
                res += self[(0, c)] * self.cofactor(0, c)
            }
            res
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut res = Matrix::zeroed(self.height - 1, self.width - 1);
        let mut self_r = 0;
        for r in 0..res.height {
            if self_r == row {
                self_r += 1
            }
            let mut self_c = 0;
            for c in 0..res.width {
                if self_c == col {
                    self_c += 1
                }
                res[(r, c)] = self[(self_r, self_c)];
                self_c += 1
            }
            self_r += 1
        }
        res
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).det()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        self.minor(row, col) * if (row + col) % 2 == 0 { 1. } else { -1. }
    }

    pub fn is_invertible(&self) -> bool {
        self.det() != 0.
    }

    pub fn inverse(&self) -> Self {
        let det = self.det();

        let mut res = Matrix::zeroed(self.height, self.width);
        for row in 0..res.height {
            for col in 0..res.width {
                res[(row, col)] = self.cofactor(col, row)/det
            }
        }
        res
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
        let matrix = Matrix::from_rows(2, 2, vec![vec![-3., 5.], vec![1., -2.]]);
        assert_abs_diff_eq!(matrix[(0, 0)], -3.);
        assert_abs_diff_eq!(matrix[(0, 1)], 5.);
        assert_abs_diff_eq!(matrix[(1, 0)], 1.);
        assert_abs_diff_eq!(matrix[(1, 1)], -2.);
    }

    #[test]
    fn it_creates_3_by_3_matrices() {
        let matrix = Matrix::from_rows(
            3,
            3,
            vec![vec![-3., 5., 0.], vec![1., -2., 7.], vec![0., 1., 1.]],
        );
        assert_abs_diff_eq!(matrix[(0, 0)], -3.);
        assert_abs_diff_eq!(matrix[(1, 1)], -2.);
        assert_abs_diff_eq!(matrix[(2, 2)], 1.);
    }

    #[test]
    fn it_creates_4_by_4_matrices() {
        let matrix = Matrix::from_rows(
            4,
            4,
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
            4,
            4,
            vec![
                vec![1., 2., 3., 4.],
                vec![5., 6., 7., 8.],
                vec![9., 8., 7., 6.],
                vec![5., 4., 3., 2.],
            ],
        );
        let matrix2 = Matrix::from_rows(
            4,
            4,
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
            4,
            4,
            vec![
                vec![1., 2., 3., 4.],
                vec![5., 6., 7., 8.],
                vec![9., 8., 7., 6.],
                vec![5., 4., 3., 2.],
            ],
        );
        let matrix2 = Matrix::from_rows(
            4,
            4,
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
            4,
            4,
            vec![
                vec![1., 2., 3., 4.],
                vec![5., 6., 7., 8.],
                vec![9., 8., 7., 6.],
                vec![5., 4., 3., 2.],
            ],
        );
        let matrix2 = Matrix::from_rows(
            4,
            4,
            vec![
                vec![-2., 1., 2., 3.],
                vec![3., 2., 1., -1.],
                vec![4., 3., 6., 5.],
                vec![1., 2., 7., 8.],
            ],
        );
        let expected = Matrix::from_rows(
            4,
            4,
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
            4,
            4,
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
            4,
            4,
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

    #[test]
    fn it_transpose_matrices() {
        let matrix = Matrix::from_rows(
            4,
            4,
            vec![
                vec![0., 9., 3., 0.],
                vec![9., 8., 0., 8.],
                vec![1., 8., 5., 3.],
                vec![0., 0., 5., 8.],
            ],
        );
        let expected = Matrix::from_rows(
            4,
            4,
            vec![
                vec![0., 9., 1., 0.],
                vec![9., 8., 8., 0.],
                vec![3., 0., 5., 5.],
                vec![0., 8., 3., 8.],
            ],
        );
        assert_abs_diff_eq!(matrix.transposed(), expected);
    }

    #[test]
    fn it_transpose_the_identity_matrix() {
        let identity = Matrix::identity(4);
        assert_abs_diff_eq!(identity.transposed(), identity);
    }

    #[test]
    fn it_calculate_the_determinant_of_a_2x2_matrix() {
        let matrix = Matrix::from_rows(2, 2, vec![vec![1., 5.], vec![-3., 2.]]);
        assert_abs_diff_eq!(matrix.det(), 17.);
    }

    #[test]
    fn it_extract_a_2x2_submatrix_from_a_3x3_matrix() {
        let matrix = Matrix::from_rows(
            3,
            3,
            vec![vec![1., 5., 0.], vec![-3., 2., 7.], vec![0., 6., -3.]],
        );
        let expected = Matrix::from_rows(2, 2, vec![vec![-3., 2.], vec![0., 6.]]);
        assert_abs_diff_eq!(matrix.submatrix(0, 2), expected);
    }

    #[test]
    fn it_extract_a_3x3_submatrix_from_a_4x4_matrix() {
        let matrix = Matrix::from_rows(
            4,
            4,
            vec![
                vec![-6., 1., 1., 6.],
                vec![-8., 5., 8., 6.],
                vec![-1., 0., 8., 2.],
                vec![-7., 1., -1., 1.],
            ],
        );
        let expected = Matrix::from_rows(
            3,
            3,
            vec![vec![-6., 1., 6.], vec![-8., 8., 6.], vec![-7., -1., 1.]],
        );
        assert_abs_diff_eq!(matrix.submatrix(2, 1), expected);
    }

    #[test]
    fn it_calculate_the_minor_of_a_3x3_matrix() {
        let matrix = Matrix::from_rows(
            3,
            3,
            vec![vec![3., 5., 0.], vec![2., -1., -7.], vec![6., -1., 5.]],
        );
        assert_abs_diff_eq!(matrix.minor(1, 0), 25.);
    }

    #[test]
    fn it_calculate_the_cofactor_of_a_3x3_matrix() {
        let matrix = Matrix::from_rows(
            3,
            3,
            vec![vec![3., 5., 0.], vec![2., -1., -7.], vec![6., -1., 5.]],
        );
        assert_abs_diff_eq!(matrix.minor(0, 0), -12.);
        assert_abs_diff_eq!(matrix.cofactor(0, 0), -12.);
        assert_abs_diff_eq!(matrix.minor(1, 0), 25.);
        assert_abs_diff_eq!(matrix.cofactor(1, 0), -25.);
    }

    #[test]
    fn it_calculate_the_determinant_of_a_3x3_matrix() {
        let matrix = Matrix::from_rows(
            3,
            3,
            vec![vec![1., 2., 6.], vec![-5., 8., -4.], vec![2., 6., 4.]],
        );
        assert_abs_diff_eq!(matrix.cofactor(0, 0), 56.);
        assert_abs_diff_eq!(matrix.cofactor(0, 1), 12.);
        assert_abs_diff_eq!(matrix.cofactor(0, 2), -46.);
        assert_abs_diff_eq!(matrix.det(), -196.);
    }

    #[test]
    fn it_calculate_the_determinant_of_a_4x4_matrix() {
        let matrix = Matrix::from_rows(
            4,
            4,
            vec![
                vec![-2., -8., 3., 5.],
                vec![-3., 1., 7., 3.],
                vec![1., 2., -9., 6.],
                vec![-6., 7., 7., -9.],
            ],
        );
        assert_abs_diff_eq!(matrix.cofactor(0, 0), 690.);
        assert_abs_diff_eq!(matrix.cofactor(0, 1), 447.);
        assert_abs_diff_eq!(matrix.cofactor(0, 2), 210.);
        assert_abs_diff_eq!(matrix.cofactor(0, 3), 51.);
        assert_abs_diff_eq!(matrix.det(), -4071.);
    }

    #[test]
    fn it_tests_if_a_4x4_matrix_is_invertible() {
        let matrix = Matrix::from_rows(
            4,
            4,
            vec![
                vec![6., 4., 4., 4.],
                vec![5., 5., 7., 6.],
                vec![4., -9., 3., -7.],
                vec![9., 1., 7., -6.],
            ],
        );
        assert_abs_diff_eq!(matrix.det(), -2120.);
        assert!(matrix.is_invertible());
    }

    #[test]
    fn it_tests_if_a_4x4_matrix_is_not_invertible() {
        let matrix = Matrix::from_rows(
            4,
            4,
            vec![
                vec![-4., 2., -2., 3.],
                vec![9., 6., 2., 6.],
                vec![0., -5., 1., -5.],
                vec![0., 0., 0., 0.],
            ],
        );
        assert_abs_diff_eq!(matrix.det(), 0.);
        assert_eq!(matrix.is_invertible(), false);
    }

    #[test]
    fn it_calculates_the_inverse_of_a_4x4_matrix() {
        let matrix = Matrix::from_rows(
            4,
            4,
            vec![
                vec![-5., 2., 6., -8.],
                vec![1., -5., 1., 8.],
                vec![7., 7., -6., -7.],
                vec![1., -3., 7., 4.],
            ],
        );

        let expected = Matrix::from_rows(
            4,
            4,
            vec![
                vec![116./532., 240./532., 128./532., -24./532.],
                vec![-430./532., -775./532., -236./532., 277./532.],
                vec![-42./532., -119./532., -28./532., 105./532.],
                vec![-278./532., -433./532., -160./532., 163./532.],
            ],
        );
        assert_abs_diff_eq!(matrix.det(), 532.);
        assert_abs_diff_eq!(matrix.cofactor(2, 3), -160.);
        assert_abs_diff_eq!(matrix.cofactor(3, 2), 105.);
        assert_abs_diff_eq!(matrix.inverse(), expected);
    }

    #[test]
    fn it_multuply_a_product_by_its_inverse() {
        let matrix1 = Matrix::from_rows(
            4,
            4,
            vec![
                vec![3., -9., 7., 3.],
                vec![3., -8., 2., -9.],
                vec![-4., 4., 4., 1.],
                vec![-6., 5., -1., 1.],
            ],
        );

        let matrix2 = Matrix::from_rows(
            4,
            4,
            vec![
                vec![8., 2., 2., 2.],
                vec![3., -1., 7., 0.],
                vec![7., 0., 5., 4.],
                vec![6., -2., 0., 5.],
            ],
        );
        
        assert_abs_diff_eq!(&(&matrix1 * &matrix2) * &matrix2.inverse(), matrix1);
    }
}
