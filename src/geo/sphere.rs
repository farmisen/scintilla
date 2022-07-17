use crate::geo::Ray;
use crate::geo::{Intersectable, Intersection, Intersections};
use crate::matrix::Matrix4;
use crate::tuple::{Point3, Vector3};
use approx::AbsDiffEq;
use std::any::Any;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub origin: Point3,
    pub radius: f64,
    pub transform: Matrix4, // put it in intersectable?
}

impl Sphere {
    pub fn new(origin: Point3, radius: f64) -> Self {
        Self {
            origin,
            radius,
            transform: Matrix4::identity(),
        }
    }

    pub fn unit() -> Self {
        Self {
            origin: Point3::origin(),
            radius: 1.0,
            transform: Matrix4::identity()
        }
    }

    pub fn intersections(&self, ray: &Ray) -> Intersections {
        // println!("Sphere:{:?}", self);
        // println!("Ray:{:?}", ray);
        let transformed_ray = ray.transform(&self.transform.inversed());
        let sphere_to_ray: Vector3 = transformed_ray.origin - self.origin;
        let a = transformed_ray.direction.dot(&transformed_ray.direction);
        let b = transformed_ray.direction.dot(&sphere_to_ray) * 2.;
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
        let d = b * b - 4. * a * c;
        Intersections::new(if d < 0. {
            vec![]
        } else {
            vec![
                Intersection::new((-b - d.sqrt()) / (2. * a), Intersectable::Sphere(*self)),
                Intersection::new((-b + d.sqrt()) / (2. * a), Intersectable::Sphere(*self)),
            ]
        })
    }
}

impl AbsDiffEq for Sphere {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        Point3::abs_diff_eq(&self.origin, &other.origin, epsilon)
            && f64::abs_diff_eq(&self.radius, &other.radius, epsilon)
    }
}


#[cfg(test)]
mod tests {
    use super::Sphere;
    use crate::geo::{Intersectable, Ray};
    use crate::matrix::Matrix4;
    use crate::tuple::{Point3, Vector3};

    #[test]
    fn it_sets_the_object_on_the_intersection() {
        let r = Ray::new(Point3::point(0., 0., -5.), Vector3::vector(0., 0., 1.));
        let s = Sphere::unit();
        let i = Intersectable::Sphere(s);
        let xs = r.intersect(&i);
        assert_eq!(xs.count(), 2);
        let Intersectable::Sphere(s1) = xs[0].intersectable;
        assert_abs_diff_eq!(s, s1);
        let Intersectable::Sphere(s2) = xs[1].intersectable;
        assert_abs_diff_eq!(s, s2);
    }

    #[test]
    fn it_has_a_default_transformation() {
        let s = Sphere::unit();
        assert_abs_diff_eq!(s.transform, Matrix4::identity());
    }

    #[test]
    fn it_can_intersect_with_a_ray_when_scaled() {
        let r = Ray::new(Point3::point(0., 0., -5.), Vector3::vector(0., 0., 1.));
        let mut s = Sphere::unit();
        s.transform = Matrix4::scale_matrix(2., 2.,2.);
        let xs = s.intersections(&r);
        assert_eq!(xs.count(), 2);
        assert_abs_diff_eq!(xs[0].t, 3.);
        assert_abs_diff_eq!(xs[1].t, 7.);
    }


    #[test]
    fn it_can_intersect_with_a_ray_when_translated() {
        let r = Ray::new(Point3::point(0., 0., -5.), Vector3::vector(0., 0., 1.));
        let mut s = Sphere::unit();
        s.transform = Matrix4::translation_matrix(5., 0.,0.);
        let xs = s.intersections(&r);
        assert_eq!(xs.count(), 0);
    }



}
