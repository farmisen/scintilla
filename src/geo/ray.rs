use crate::geo::{Intersectable, Intersection, Intersections, Sphere};
use crate::matrix::Matrix;
use crate::tuple::{Point3, Vector3};
use approx::AbsDiffEq;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, distance: f64) -> Point3 {
        self.origin + self.direction * distance
    }

    pub fn intersect(&self, intersectable: &Intersectable) -> Intersections {
        intersectable.intersections(&self)
    }

    pub fn transform(&self, matrix: &Matrix<4, 4>) -> Self {
        Self {
            origin:  matrix * self.origin,
            direction: matrix * self.direction
        }
    }
}

impl AbsDiffEq for Ray {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        Point3::abs_diff_eq(&self.origin, &other.origin, epsilon)
            && Vector3::abs_diff_eq(&self.direction, &other.direction, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::geo::{Intersectable, Intersection, Sphere};
    use crate::matrix::Matrix;
    use crate::tuple::{Point3, Vector3};

    #[test]
    fn it_create_rays() {
        let origin = Point3::point(1., 2., 3.);
        let direction = Vector3::vector(4., 5., 6.);
        let ray = Ray::new(origin, direction);
        assert_abs_diff_eq!(origin, ray.origin);
        assert_abs_diff_eq!(direction, ray.direction);
    }

    #[test]
    fn it_computes_a_point_from_a_distance() {
        let ray = Ray::new(Point3::point(2., 3., 4.), Vector3::vector(1., 0., 0.));
        assert_abs_diff_eq!(ray.position(0.), Point3::point(2., 3., 4.));
        assert_abs_diff_eq!(ray.position(1.), Point3::point(3., 3., 4.));
        assert_abs_diff_eq!(ray.position(-1.), Point3::point(1., 3., 4.));
        assert_abs_diff_eq!(ray.position(2.5), Point3::point(4.5, 3., 4.));
    }

    #[test]
    fn it_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point3::point(0., 0., -5.), Vector3::vector(0., 0., 1.));
        let s = Sphere::new(Point3::point(0., 0., 0.), 1.);
        let i = Intersectable::Sphere(s);
        let xs = r.intersect(&i);
        assert_eq!(xs.count(), 2);
        assert_abs_diff_eq!(xs[0].t, 4.0);
        assert_abs_diff_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn it_intersects_a_sphere_at_its_tangent() {
        let r = Ray::new(Point3::point(0., 1., -5.), Vector3::vector(0., 0., 1.));
        let s = Sphere::new(Point3::point(0., 0., 0.), 1.);
        let i = Intersectable::Sphere(s);
        let xs = r.intersect(&i);
        assert_eq!(xs.count(), 2);
        assert_abs_diff_eq!(xs[0].t, 5.0);
        assert_abs_diff_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn it_misses_a_sphere() {
        let r = Ray::new(Point3::point(0., 2., -5.), Vector3::vector(0., 0., 1.));
        let s = Sphere::new(Point3::point(0., 0., 0.), 1.);
        let i = Intersectable::Sphere(s);
        let xs = r.intersect(&i);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn it_intersects_a_sphere_at_two_points_when_originating_from_inside_the_sphere() {
        let r = Ray::new(Point3::point(0., 0., 0.), Vector3::vector(0., 0., 1.));
        let s = Sphere::new(Point3::point(0., 0., 0.), 1.);
        let i = Intersectable::Sphere(s);
        let xs = r.intersect(&i);
        assert_eq!(xs.count(), 2);
        assert_abs_diff_eq!(xs[0].t, -1.0);
        assert_abs_diff_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn it_intersects_a_sphere_at_two_points_when_the_sphere_is_behind_the_ray() {
        let r = Ray::new(Point3::point(0., 0., 5.), Vector3::vector(0., 0., 1.));
        let s = Sphere::new(Point3::point(0., 0., 0.), 1.);
        let i = Intersectable::Sphere(s);
        let xs = r.intersect(&i);
        assert_eq!(xs.count(), 2);
        assert_abs_diff_eq!(xs[0].t, -6.0);
        assert_abs_diff_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn it_can_be_translated() {
        let r1 = Ray::new(Point3::point(1., 2., 3.), Vector3::vector(0., 1., 0.));
        let m = Matrix::translation_matrix(3., 4., 5.);
        let r2 = r1.transform(&m);
        assert_abs_diff_eq!(r2.origin, Point3::point(4., 6., 8.));
        assert_abs_diff_eq!(r2.direction, Vector3::vector(0., 1., 0.));
    }

    #[test]
    fn it_can_be_scaled() {
        let r1 = Ray::new(Point3::point(1., 2., 3.), Vector3::vector(0., 1., 0.));
        let m = Matrix::scale_matrix(2., 3., 4.);
        let r2 = r1.transform(&m);
        assert_abs_diff_eq!(r2.origin, Point3::point(2., 6., 12.));
        assert_abs_diff_eq!(r2.direction, Vector3::vector(0., 3., 0.));
    }
}
