use crate::matrix::Matrix4;

impl Matrix4 {
    pub fn translation_matrix(x: f64, y: f64, z: f64) -> Self {
        let mut matrix = Matrix4::identity();
        matrix[(0, 3)] = x;
        matrix[(1, 3)] = y;
        matrix[(2, 3)] = z;
        matrix
    }

  
    pub fn scale_matrix(x: f64, y: f64, z: f64) -> Self {
        let mut matrix = Matrix4::identity();
        matrix[(0, 0)] = x;
        matrix[(1, 1)] = y;
        matrix[(2, 2)] = z;
        matrix
    }

    pub fn rotation_x_matrix(a: f64) -> Self {
        Matrix4::from_rows([
            [1., 0., 0., 0.],
            [0., a.cos(), -a.sin(), 0.],
            [0., a.sin(), a.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotation_y_matrix(a: f64) -> Self {
        Matrix4::from_rows([
            [a.cos(), 0., a.sin(), 0.],
            [0., 1., 0., 0.],
            [-a.sin(), 0., a.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn rotation_z_matrix(a: f64) -> Self {
        Matrix4::from_rows([
            [a.cos(), -a.sin(), 0., 0.],
            [a.sin(), a.cos(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn shear_matrix(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Matrix4::from_rows([
            [1., xy, xz, 0.],
            [yx, 1., yz, 0.],
            [zx, zy, 1., 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Self {
         Matrix4::translation_matrix(x,y,z) * self
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        Matrix4::scale_matrix(x,y,z) * self
    }

    pub fn rotate_x(&self, a: f64) -> Self {
        Matrix4::rotation_x_matrix(a) * self
    }

    pub fn rotate_y(&self, a: f64) -> Self {
        Matrix4::rotation_y_matrix(a) * self
    }

    pub fn rotate_z(&self, a: f64) -> Self {
        Matrix4::rotation_z_matrix(a) * self
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::matrix::Matrix4;
    use crate::tuple::Point3;
    use crate::tuple::Vector3;

    #[test]
    fn it_translates_points() {
        let matrix = Matrix4::translation_matrix(5., -3., 2.);
        let point = Point3::point(-3., 4., 5.);
        let expected = Point3::point(2., 1., 7.);
        assert_abs_diff_eq!(matrix * point, expected);
    }

    #[test]
    fn it_inversely_translates_points() {
        let matrix = Matrix4::translation_matrix(5., -3., 2.);
        let point = Point3::point(-3., 4., 5.);
        let expected = Point3::point(-8., 7., 3.);
        assert_abs_diff_eq!(matrix.inversed() * point, expected);
    }

    #[test]
    fn it_does_not_translate_vectors() {
        let matrix = Matrix4::translation_matrix(5., -3., 2.);
        let vec = Vector3::vector(-3., 4., 5.);
        assert_abs_diff_eq!(matrix * vec, vec);
    }

    #[test]
    fn it_scales_points() {
        let matrix = Matrix4::scale_matrix(2., 3., 4.);
        let point = Point3::point(-4., 6., 8.);
        let expected = Point3::point(-8., 18., 32.);
        assert_abs_diff_eq!(matrix * point, expected);
    }

    #[test]
    fn it_scales_vectors() {
        let matrix = Matrix4::scale_matrix(2., 3., 4.);
        let vec = Vector3::vector(-4., 6., 8.);
        let expected = Vector3::vector(-8., 18., 32.);
        assert_abs_diff_eq!(matrix * vec, expected);
    }

    #[test]
    fn it_inversely_scales() {
        let matrix = Matrix4::scale_matrix(2., 3., 4.);
        let point = Point3::point(-4., 6., 8.);
        let expected = Point3::point(-2., 2., 2.);
        assert_abs_diff_eq!(matrix.inversed() * point, expected);
    }

    #[test]
    fn it_reflect_by_scaling_by_a_negative_value() {
        let matrix = Matrix4::scale_matrix(-1., 1., 1.);
        let point = Point3::point(2., 3., 4.);
        let expected = Point3::point(-2., 3., 4.);
        assert_abs_diff_eq!(matrix * point, expected);
    }

    #[test]
    fn it_rotates_a_point_around_the_x_axis() {
        let half_quarter = Matrix4::rotation_x_matrix(PI / 4.);
        let full_quarter = Matrix4::rotation_x_matrix(PI / 2.);
        let point = Point3::point(0., 1., 0.);
        assert_abs_diff_eq!(
            half_quarter * point,
            Point3::point(0., 2.0_f64.sqrt() / 2., 2.0_f64.sqrt() / 2.)
        );
        assert_abs_diff_eq!(full_quarter * point, Point3::point(0., 0., 1.));
    }

    #[test]
    fn it_rotates_a_point_around_the_y_axis() {
        let half_quarter = Matrix4::rotation_y_matrix(PI / 4.);
        let full_quarter = Matrix4::rotation_y_matrix(PI / 2.);
        let point = Point3::point(0., 0., 1.);
        assert_abs_diff_eq!(
            half_quarter * point,
            Point3::point(2.0_f64.sqrt() / 2., 0., 2.0_f64.sqrt() / 2.)
        );
        assert_abs_diff_eq!(full_quarter * point, Point3::point(1., 0., 0.));
    }

    #[test]
    fn it_rotates_a_point_around_the_z_axis() {
        let half_quarter = Matrix4::rotation_z_matrix(PI / 4.);
        let full_quarter = Matrix4::rotation_z_matrix(PI / 2.);
        let point = Point3::point(0., 1., 0.);
        assert_abs_diff_eq!(
            half_quarter * point,
            Point3::point(-2.0_f64.sqrt() / 2., 2.0_f64.sqrt() / 2., 0.)
        );
        assert_abs_diff_eq!(full_quarter * point, Point3::point(-1., 0., 0.));
    }

    #[test]
    fn it_shears_x_in_proportion_to_y() {
        let shearing = Matrix4::shear_matrix(1., 0., 0., 0., 0., 0.);
        let point = Point3::point(2., 3., 4.);
        assert_abs_diff_eq!(shearing * point, Point3::point(5., 3., 4.));
    }

    #[test]
    fn it_shears_x_in_proportion_to_z() {
        let shearing = Matrix4::shear_matrix(0., 1., 0., 0., 0., 0.);
        let point = Point3::point(2., 3., 4.);
        assert_abs_diff_eq!(shearing * point, Point3::point(6., 3., 4.));
    }

    #[test]
    fn it_shears_y_in_proportion_to_x() {
        let shearing = Matrix4::shear_matrix(0., 0., 1., 0., 0., 0.);
        let point = Point3::point(2., 3., 4.);
        assert_abs_diff_eq!(shearing * point, Point3::point(2., 5., 4.));
    }

    #[test]
    fn it_shears_y_in_proportion_to_z() {
        let shearing = Matrix4::shear_matrix(0., 0., 0., 1., 0., 0.);
        let point = Point3::point(2., 3., 4.);
        assert_abs_diff_eq!(shearing * point, Point3::point(2., 7., 4.));
    }

    #[test]
    fn it_shears_z_in_proportion_to_x() {
        let shearing = Matrix4::shear_matrix(0., 0., 0., 0., 1., 0.);
        let point = Point3::point(2., 3., 4.);
        assert_abs_diff_eq!(shearing * point, Point3::point(2., 3., 6.));
    }

    #[test]
    fn it_shears_z_in_proportion_to_y() {
        let shearing = Matrix4::shear_matrix(0., 0., 0., 0., 0., 1.);
        let point = Point3::point(2., 3., 4.);
        assert_abs_diff_eq!(shearing * point, Point3::point(2., 3., 7.));
    }

    #[test]
    fn it_applies_individual_transformations_in_sequence() {
        let point1 = Point3::point(1., 0., 1.);
        let rot_mat = Matrix4::rotation_x_matrix(PI / 2.);
        let scal_mat = Matrix4::scale_matrix(5., 5., 5.);
        let trans_mat = Matrix4::translation_matrix(10., 5., 7.);
        let point2 = rot_mat * point1;
        let point3 = scal_mat * point2;
        let point4 = trans_mat * point3;
        assert_abs_diff_eq!(point4, Point3::point(15., 0., 7.));
    }

    #[test]
    fn it_applies_chained_transformation_in_reverse_order() {
        let point1 = Point3::point(1., 0., 1.);
        let rot_mat = Matrix4::rotation_x_matrix(PI / 2.);
        let scal_mat = Matrix4::scale_matrix(5., 5., 5.);
        let trans_mat = Matrix4::translation_matrix(10., 5., 7.);
        let mat = trans_mat * scal_mat * rot_mat;
        let point2 = mat * point1;
        assert_abs_diff_eq!(point2, Point3::point(15., 0., 7.));
    }

    #[test]
    fn it_is_fluent() {
        let point1 = Point3::point(1., 0., 1.);
        let matrix = Matrix4::identity()
            .rotate_x(PI / 2.)
            .scale(5., 5., 5.)
            .translate(10., 5., 7.);
        let point2 = matrix * point1;
        assert_abs_diff_eq!(point2, Point3::point(15., 0., 7.));
    }
}
