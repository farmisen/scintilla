use crate::geo::{Intersections, Intersection, Intersectable, Sphere};
use crate::tuple::Point3;
use crate::tuple::Vector3;
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

    pub fn intersect(&self, intersectable: Box<dyn Intersectable>) -> Intersections {
        Intersections::new(intersectable.intersections(&self))
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
    use crate::tuple::{Point3, Vector3};
    use crate::geo::{Intersection, Intersectable, Sphere};


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
        let xs = r.intersect(Box::new(s));
        assert_eq!(xs.count(), 2);
        assert_abs_diff_eq!(xs[0].t, 4.0);
        assert_abs_diff_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn it_intersects_a_sphere_at_its_tangent() {
        let r = Ray::new(Point3::point(0., 1., -5.), Vector3::vector(0., 0., 1.));
        let s = Sphere::new(Point3::point(0., 0., 0.), 1.);
        let xs = r.intersect(Box::new(s));
        assert_eq!(xs.count(), 2);
        assert_abs_diff_eq!(xs[0].t, 5.0);
        assert_abs_diff_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn it_misses_a_sphere() {
        let r = Ray::new(Point3::point(0., 2., -5.), Vector3::vector(0., 0., 1.));
        let s = Sphere::new(Point3::point(0., 0., 0.), 1.);
        let xs = r.intersect(Box::new(s));
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn it_intersects_a_sphere_at_two_points_when_originating_from_inside_the_sphere() {
        let r = Ray::new(Point3::point(0., 0., 0.), Vector3::vector(0., 0., 1.));
        let s = Sphere::new(Point3::point(0., 0., 0.), 1.);
        let xs = r.intersect(Box::new(s));
        assert_eq!(xs.count(), 2);
        assert_abs_diff_eq!(xs[0].t, -1.0);
        assert_abs_diff_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn it_intersects_a_sphere_at_two_points_when_the_sphere_is_behind_the_ray() {
        let r = Ray::new(Point3::point(0., 0., 5.), Vector3::vector(0., 0., 1.));
        let s= Sphere::new(Point3::point(0., 0., 0.), 1.);
        let xs = r.intersect(Box::new(s));
        assert_eq!(xs.count(), 2);
        assert_abs_diff_eq!(xs[0].t, -6.0);
        assert_abs_diff_eq!(xs[1].t, -4.0);
    }
}
