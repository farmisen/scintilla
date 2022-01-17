use crate::tuple::Tuple;
use approx::AbsDiffEq;
use std::fmt;
use std::ops::{Index, IndexMut, Mul};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const W: usize, const H: usize> {
    pub(in crate::matrix) data: [[f64; W]; H],
}

pub type Matrix2 = Matrix<2, 2>;
pub type Matrix3 = Matrix<3, 3>;
pub type Matrix4 = Matrix<4, 4>;

impl<const W: usize, const H: usize> Default for Matrix<W, H> {
    fn default() -> Self {
        Matrix::<W, H>::from_rows([[0.0; W]; H])
    }
}

impl<const W: usize, const H: usize> Matrix<W, H> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: [[f64; W]; H]) -> Self {
        Self { data: rows }
    }

    pub fn from_tuple(tuple: Tuple) -> Matrix<1, 4> {
        Matrix::<1, 4> {
            data: [[tuple.x], [tuple.y], [tuple.z], [tuple.w]],
        }
    }
}

impl<const W: usize, const H: usize> Index<(usize, usize)> for Matrix<W, H> {
    type Output = f64;

    // (row, col)
    fn index(&self, indice: (usize, usize)) -> &Self::Output {
        &self.data[indice.0][indice.1]
    }
}

impl<const W: usize, const H: usize> IndexMut<(usize, usize)> for Matrix<W, H> {
    // (row, col)
    fn index_mut(&mut self, indice: (usize, usize)) -> &mut Self::Output {
        &mut self.data[indice.0][indice.1]
    }
}

impl<const W: usize, const H: usize, const OW: usize, const OH: usize> Mul<Matrix<OW, OH>>
    for Matrix<W, H>
{
    type Output = Matrix<OW, H>;

    fn mul(self, other: Matrix<OW, OH>) -> Matrix<OW, H> {
        let mut matrix = Matrix::<OW, H>::default();
        for row in 0..H {
            for col in 0..OW {
                let mut value = 0.0;
                for i in 0..W {
                    value += self[(row, i)] * other[(i, col)]
                }
                matrix[(row, col)] = value
            }
        }

        matrix
    }
}

impl<const W: usize, const H: usize, const OW: usize, const OH: usize> Mul<Matrix<OW, OH>>
    for &Matrix<W, H>
{
    type Output = Matrix<OW, H>;

    fn mul(self, other: Matrix<OW, OH>) -> Matrix<OW, H> {
        let mut matrix = Matrix::<OW, H>::default();
        for row in 0..H {
            for col in 0..OW {
                let mut value = 0.0;
                for i in 0..W {
                    value += self[(row, i)] * other[(i, col)]
                }
                matrix[(row, col)] = value
            }
        }

        matrix
    }
}

impl<const W: usize, const H: usize, const OW: usize, const OH: usize> Mul<&Matrix<OW, OH>>
    for Matrix<W, H>
{
    type Output = Matrix<OW, H>;

    fn mul(self, other: &Matrix<OW, OH>) -> Matrix<OW, H> {
        let mut matrix = Matrix::<OW, H>::default();
        for row in 0..H {
            for col in 0..OW {
                let mut value = 0.0;
                for i in 0..W {
                    value += self[(row, i)] * other[(i, col)]
                }
                matrix[(row, col)] = value
            }
        }

        matrix
    }
}

impl Mul<Tuple> for Matrix<4, 4> {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let other_matrix = Matrix::<4,4>::from_tuple(other);
        let res = self * other_matrix;
        Tuple::new(res[(0, 0)], res[(1, 0)], res[(2, 0)], res[(3, 0)])
    }
}

impl Mul<Tuple> for &Matrix<4, 4> {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let other = Matrix::<4,4>::from_tuple(other);
        let res = self * other;
        Tuple::new(res[(0, 0)], res[(1, 0)], res[(2, 0)], res[(3, 0)])
    }
}

impl<const W: usize, const H: usize> AbsDiffEq for Matrix<W, H> {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1.0E-14f64
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        self.data
            .iter()
            .zip(other.data.iter())
            .fold(true, |accu, (a, b)| {
                accu && a
                    .iter()
                    .zip(b)
                    .fold(true, |accu2, (s, t)| accu2 && f64::abs_diff_eq(&s, &t, epsilon))
            })
    }
}

impl Matrix4 {
    pub fn identity() -> Self {
        let mut res = Matrix4::new();
        for i in 0..4 {
            res[(i, i)] = 1.;
        }
        res
    }
}

impl Matrix3 {
    pub fn identity() -> Self {
        let mut res = Matrix3::new();
        for i in 0..3 {
            res[(i, i)] = 1.;
        }
        res
    }
}

impl Matrix2 {
    pub fn identity() -> Self {
        let mut res = Matrix2::new();
        for i in 0..2 {
            res[(i, i)] = 1.;
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::{Matrix, Matrix2, Matrix3, Matrix4};
    use crate::tuple::Tuple;

    #[test]
    fn it_creates_2_by_2_matrices() {
        let matrix = Matrix2::from_rows([[-3., 5.], [1., -2.]]);
        assert_abs_diff_eq!(matrix[(0, 0)], -3.);
        assert_abs_diff_eq!(matrix[(0, 1)], 5.);
        assert_abs_diff_eq!(matrix[(1, 0)], 1.);
        assert_abs_diff_eq!(matrix[(1, 1)], -2.);
    }

    #[test]
    fn it_creates_3_by_3_matrices() {
        let matrix = Matrix3::from_rows([[-3., 5., 0.], [1., -2., 7.], [0., 1., 1.]]);
        assert_abs_diff_eq!(matrix[(0, 0)], -3.);
        assert_abs_diff_eq!(matrix[(1, 1)], -2.);
        assert_abs_diff_eq!(matrix[(2, 2)], 1.);
    }

    #[test]
    fn it_creates_4_by_4_matrices() {
        let matrix = Matrix4::from_rows([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 1.],
            [13.5, 14.5, 15.5, 16.5],
        ]);
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
        let matrix1 = Matrix4::from_rows([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        let matrix2 = Matrix4::from_rows([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        assert_abs_diff_eq!(matrix1, matrix2);
    }

    #[test]
    fn it_asserts_as_not_equal_different_matrices() {
        let matrix1 = Matrix4::from_rows([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        let matrix2 = Matrix4::from_rows([
            [0., 2., 3., 4.],
            [5., 0., 7., 8.],
            [9., 8., 0., 6.],
            [5., 4., 3., 0.],
        ]);
        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn it_multiply_matrices() {
        let matrix1 = Matrix4::from_rows([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        let matrix2 = Matrix4::from_rows([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);
        let expected = Matrix4::from_rows([
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.],
        ]);
        assert_abs_diff_eq!(matrix1 * matrix2, expected);
    }

    #[test]
    fn it_multiply_matrices_and_tuples() {
        let matrix = Matrix4::from_rows([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        ]);
        let tuple = Tuple::new(1., 2., 3., 1.);
        let expected = Tuple::new(18., 24., 33., 1.);
        assert_abs_diff_eq!(matrix * tuple, expected);
    }

    #[test]
    fn it_can_be_multiplied_by_the_identity_matrix() {
        let matrix = Matrix4::from_rows([
            [0., 1., 2., 4.],
            [1., 2., 4., 8.],
            [2., 4., 8., 16.],
            [4., 8., 16., 32.],
        ]);
        let identity = Matrix4::identity();
        let expected = matrix.clone();
        assert_abs_diff_eq!(matrix * identity, expected);
    }
}
