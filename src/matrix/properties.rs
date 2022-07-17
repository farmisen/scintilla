use crate::matrix::Matrix;

impl Matrix<2, 2> {
    pub fn det(&self) -> f64 {
        self[(0, 0)] * self[(1, 1)] - self[(1, 0)] * self[(0, 1)]
    }
}

impl Matrix<3, 3> {
    pub fn det(&self) -> f64 {
        let mut res = 0.;
        for c in 0..3 {
            res += self[(0, c)] * self.cofactor(0, c)
        }
        res
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        self.minor(row, col) * if (row + col) % 2 == 0 { 1. } else { -1. }
    }

    pub fn inversed(&self) -> Self {
        let det = self.det();

        let mut res = Matrix::<3, 3>::new();
        for row in 0..3 {
            for col in 0..3 {
                res[(row, col)] = self.cofactor(col, row) / det
            }
        }
        res
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).det()
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<2, 2> {
        let mut res = Matrix::<2, 2>::default();
        let mut self_r = 0;
        for r in 0..2 {
            if self_r == row {
                self_r += 1
            }
            let mut self_c = 0;
            for c in 0..2 {
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
}

impl Matrix<4, 4> {
    pub fn det(&self) -> f64 {
        let mut res = 0.;
        for c in 0..4 {
            res += self[(0, c)] * self.cofactor(0, c)
        }
        res
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        self.minor(row, col) * if (row + col) % 2 == 0 { 1. } else { -1. }
    }

    pub fn is_invertible(&self) -> bool {
        self.det() != 0.
    }

    pub fn inversed(&self) -> Self {
        let det = self.det();

        let mut res = Matrix::<4, 4>::new();
        for row in 0..4 {
            for col in 0..4 {
                res[(row, col)] = self.cofactor(col, row) / det
            }
        }
        res
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).det()
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<3, 3> {
        let mut res = Matrix::<3, 3>::default();
        let mut self_r = 0;
        for r in 0..3 {
            if self_r == row {
                self_r += 1
            }
            let mut self_c = 0;
            for c in 0..3 {
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
}

impl<const W: usize, const H: usize> Matrix<W, H> {
    pub fn transposed(&self) -> Self {
        let mut res = Matrix::<W, H>::default();
        for row in 0..H {
            for col in 0..W {
                res[(row, col)] = self[(col, row)];
            }
        }
        res
    }

    // pub fn det(&self) -> f64 {
    //     let mut res = 0.;
    //     for c in 0..W {
    //         res += self[(0, c)] * self.cofactor(0, c)
    //     }
    //     res
    // }

    // pub fn submatrix(&self, row: usize, col: usize) -> Matrix<W, H> {
    //     let mut res = Matrix::zeroed(self.height - 1, self.width - 1);
    //     let mut self_r = 0;
    //     for r in 0..res.height {
    //         if self_r == row {
    //             self_r += 1
    //         }
    //         let mut self_c = 0;
    //         for c in 0..res.width {
    //             if self_c == col {
    //                 self_c += 1
    //             }
    //             res[(r, c)] = self[(self_r, self_c)];
    //             self_c += 1
    //         }
    //         self_r += 1
    //     }
    //     res
    // }
    // pub fn minor(&self, row: usize, col: usize) -> f64 {
    //     assert!(self.is_square());
    //     self.submatrix(row, col).det()
    // }
    //
    // pub fn cofactor(&self, row: usize, col: usize) -> f64 {
    //     assert!(self.is_square());
    //     self.minor(row, col) * if (row + col) % 2 == 0 { 1. } else { -1. }
    // }

    // pub fn is_invertible(&self) -> bool {
    //     self.is_square() && self.det() != 0.
    // }

    // pub fn inversed(&self) -> Self {
    //     assert!(self.is_square());
    //     let det = self.det();

    //     let mut res = Matrix::zeroed(self.height, self.width);
    //     for row in 0..res.height {
    //         for col in 0..res.width {
    //             res[(row, col)] = self.cofactor(col, row) / det
    //         }
    //     }
    //     res
    // }

    //     pub fn is_square(&self) -> bool {
    //         self.width == self.height
    //     }
    // }
}
#[cfg(test)]
mod tests {
    use crate::matrix::{Matrix2, Matrix3, Matrix4};

    #[test]
    fn it_transpose_matrices() {
        let matrix = Matrix4::from_rows([
            [0., 9., 3., 0.],
            [9., 8., 0., 8.],
            [1., 8., 5., 3.],
            [0., 0., 5., 8.],
        ]);
        let expected = Matrix4::from_rows([
            [0., 9., 1., 0.],
            [9., 8., 8., 0.],
            [3., 0., 5., 5.],
            [0., 8., 3., 8.],
        ]);
        assert_abs_diff_eq!(matrix.transposed(), expected);
    }

    #[test]
    fn it_transpose_the_identity_matrix() {
        let identity = Matrix4::identity();
        assert_abs_diff_eq!(identity.transposed(), identity);
    }

    #[test]
    fn it_calculate_the_determinant_of_a_2x2_matrix() {
        let matrix = Matrix2::from_rows([[1., 5.], [-3., 2.]]);
        assert_abs_diff_eq!(matrix.det(), 17.);
    }

    #[test]
    fn it_extract_a_2x2_submatrix_from_a_3x3_matrix() {
        let matrix = Matrix3::from_rows([[1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]]);
        let expected = Matrix2::from_rows([[-3., 2.], [0., 6.]]);
        assert_abs_diff_eq!(matrix.submatrix(0, 2), expected);
    }

    #[test]
    fn it_extract_a_3x3_submatrix_from_a_4x4_matrix() {
        let matrix = Matrix4::from_rows([
            [-6., 1., 1., 6.],
            [-8., 5., 8., 6.],
            [-1., 0., 8., 2.],
            [-7., 1., -1., 1.],
        ]);
        let expected = Matrix3::from_rows([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]]);
        assert_abs_diff_eq!(matrix.submatrix(2, 1), expected);
    }

    #[test]
    fn it_calculate_the_minor_of_a_3x3_matrix() {
        let matrix = Matrix3::from_rows([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);
        assert_abs_diff_eq!(matrix.minor(1, 0), 25.);
    }

    #[test]
    fn it_calculate_the_cofactor_of_a_3x3_matrix() {
        let matrix = Matrix3::from_rows([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);
        assert_abs_diff_eq!(matrix.minor(0, 0), -12.);
        assert_abs_diff_eq!(matrix.cofactor(0, 0), -12.);
        assert_abs_diff_eq!(matrix.minor(1, 0), 25.);
        assert_abs_diff_eq!(matrix.cofactor(1, 0), -25.);
    }

    #[test]
    fn it_calculate_the_determinant_of_a_3x3_matrix() {
        let matrix = Matrix3::from_rows([[1., 2., 6.], [-5., 8., -4.], [2., 6., 4.]]);
        assert_abs_diff_eq!(matrix.cofactor(0, 0), 56.);
        assert_abs_diff_eq!(matrix.cofactor(0, 1), 12.);
        assert_abs_diff_eq!(matrix.cofactor(0, 2), -46.);
        assert_abs_diff_eq!(matrix.det(), -196.);
    }

    #[test]
    fn it_calculate_the_determinant_of_a_4x4_matrix() {
        let matrix = Matrix4::from_rows([
            [-2., -8., 3., 5.],
            [-3., 1., 7., 3.],
            [1., 2., -9., 6.],
            [-6., 7., 7., -9.],
        ]);
        assert_abs_diff_eq!(matrix.cofactor(0, 0), 690.);
        assert_abs_diff_eq!(matrix.cofactor(0, 1), 447.);
        assert_abs_diff_eq!(matrix.cofactor(0, 2), 210.);
        assert_abs_diff_eq!(matrix.cofactor(0, 3), 51.);
        assert_abs_diff_eq!(matrix.det(), -4071.);
    }

    #[test]
    fn it_tests_if_a_4x4_matrix_is_invertible() {
        let matrix = Matrix4::from_rows([
            [6., 4., 4., 4.],
            [5., 5., 7., 6.],
            [4., -9., 3., -7.],
            [9., 1., 7., -6.],
        ]);
        assert_abs_diff_eq!(matrix.det(), -2120.);
        assert!(matrix.is_invertible());
    }

    #[test]
    fn it_tests_if_a_4x4_matrix_is_not_invertible() {
        let matrix = Matrix4::from_rows([
            [-4., 2., -2., 3.],
            [9., 6., 2., 6.],
            [0., -5., 1., -5.],
            [0., 0., 0., 0.],
        ]);
        assert_abs_diff_eq!(matrix.det(), 0.);
        assert_eq!(matrix.is_invertible(), false);
    }

    #[test]
    fn it_calculates_the_inverse_of_a_4x4_matrix() {
        let matrix = Matrix4::from_rows([
            [-5., 2., 6., -8.],
            [1., -5., 1., 8.],
            [7., 7., -6., -7.],
            [1., -3., 7., 4.],
        ]);

        let expected = Matrix4::from_rows([
            [116. / 532., 240. / 532., 128. / 532., -24. / 532.],
            [-430. / 532., -775. / 532., -236. / 532., 277. / 532.],
            [-42. / 532., -119. / 532., -28. / 532., 105. / 532.],
            [-278. / 532., -433. / 532., -160. / 532., 163. / 532.],
        ]);
        assert_abs_diff_eq!(matrix.det(), 532.);
        assert_abs_diff_eq!(matrix.cofactor(2, 3), -160.);
        assert_abs_diff_eq!(matrix.cofactor(3, 2), 105.);
        assert_abs_diff_eq!(matrix.inversed(), expected);
    }

    #[test]
    fn it_multiply_a_product_by_its_inverse() {
        let matrix1 = Matrix4::from_rows([
            [3., -9., 7., 3.],
            [3., -8., 2., -9.],
            [-4., 4., 4., 1.],
            [-6., 5., -1., 1.],
        ]);

        let matrix2 = Matrix4::from_rows([
            [8., 2., 2., 2.],
            [3., -1., 7., 0.],
            [7., 0., 5., 4.],
            [6., -2., 0., 5.],
        ]);

        let inversed = matrix2.inversed();
        let expected = matrix1.clone();

        assert_abs_diff_eq!(matrix1 * matrix2 * inversed, expected);
    }
}
