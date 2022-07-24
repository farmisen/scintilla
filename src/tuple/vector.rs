use crate::tuple::Tuple;

pub type Vector3 = Tuple;

impl Vector3 {
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Self) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, normal: Vector3) -> Vector3 {
        *self - normal * 2. * self.dot(normal)
    }
}

#[cfg(test)]
mod tests {
    use super::Vector3;
    use crate::tuple::Tuple;

    #[test]
    fn it_substracts_two_vectors() {
        let vector1 = Vector3::vector(3., 2., 1.);
        let vector2 = Vector3::vector(5., 6., 7.);
        assert_abs_diff_eq!(vector1 - vector2, Vector3::vector(-2., -4., -6.))
    }

    #[test]
    fn it_adds_two_vectors() {
        let vector1 = Vector3::vector(3., -2., 5.);
        let vector2 = Vector3::vector(-2., 3., 1.);
        assert_abs_diff_eq!(vector1 + vector2, Vector3::vector(1., 1., 6.))
    }

    #[test]
    fn it_substracts_a_vector_from_a_point() {
        let point = Vector3::point(3., 2., 1.);
        let vector = Vector3::vector(5., 6., 7.);
        assert_abs_diff_eq!(point - vector, Vector3::point(-2., -4., -6.))
    }

    #[test]
    fn it_substracts_a_vector_from_the_zero_vector() {
        let zero = Vector3::vector(0., 0., 0.);
        let vector = Vector3::vector(1., -2., 3.);
        assert_abs_diff_eq!(zero - vector, Vector3::vector(-1., 2., -3.))
    }

    #[test]
    fn it_computes_the_magnitude_of_vectors() {
        assert_abs_diff_eq!(Vector3::vector(1., 0., 0.).magnitude(), 1.);
        assert_abs_diff_eq!(Vector3::vector(0., 1., 0.).magnitude(), 1.);
        assert_abs_diff_eq!(Vector3::vector(0., 0., 1.).magnitude(), 1.);
        assert_abs_diff_eq!(Vector3::vector(1., 2., 3.).magnitude(), 14_f64.sqrt());
        assert_abs_diff_eq!(Vector3::vector(-1., -2., -3.).magnitude(), 14_f64.sqrt())
    }

    #[test]
    fn it_normalize_vectors() {
        assert_abs_diff_eq!(
            Vector3::vector(4., 0., 0.).normalized(),
            Vector3::vector(1., 0., 0.)
        );
        assert_abs_diff_eq!(
            Vector3::vector(1., 2., 3.).normalized(),
            Vector3::vector(1. / 14_f64.sqrt(), 2. / 14_f64.sqrt(), 3. / 14_f64.sqrt())
        );
        assert_abs_diff_eq!(Vector3::vector(4., 5., 6.).normalized().magnitude(), 1.);
    }

    #[test]
    fn it_computes_the_dot_product_of_two_vectors() {
        assert_abs_diff_eq!(
            Vector3::vector(1., 2., 3.).dot(Vector3::vector(2., 3., 4.)),
            20.
        );
    }

    #[test]
    fn it_computes_the_cross_product_of_two_vectors() {
        let vector1 = Vector3::vector(1., 2., 3.);
        let vector2 = Vector3::vector(2., 3., 4.);
        assert_abs_diff_eq!(vector1.cross(vector2), Vector3::vector(-1., 2., -1.));
        assert_abs_diff_eq!(vector2.cross(vector1), Vector3::vector(1., -2., 1.));
    }

    #[test]
    fn it_reflects_a_vector_approachings_at_45_degrees() {
        let v = Vector3::vector(1., -1., 0.);
        let n = Vector3::vector(0., 1., 0.);
        assert_abs_diff_eq!(v.reflect(n), Vector3::vector(1., 1., 0.));
    }

    #[test]
    fn it_reflects_a_vector_off_a_slanted_surface() {
        let v = Vector3::vector(0., -1., 0.);
        let n = Vector3::vector(f64::sqrt(2.)/2., f64::sqrt(2.)/2., 0.);
        assert_abs_diff_eq!(v.reflect(n), Vector3::vector(1., 0., 0.));
    }

}
