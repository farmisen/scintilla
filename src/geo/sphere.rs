use approx::AbsDiffEq;
use std::any::Any;
use crate::geo::{Intersectable, Intersection};
use crate::geo::Ray;
use crate::tuple::{Point3, Vector3};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub origin: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(origin: Point3, radius: f64) -> Self {
        Self { origin, radius }
    }

    pub fn unit() -> Self {
        Self { origin: Point3::origin(), radius: 1.0 }
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

impl Intersectable for Sphere {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn intersections(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray: Vector3 = ray.origin - self.origin;
        let a = ray.direction.dot(&ray.direction);
        let b = ray.direction.dot(&sphere_to_ray) * 2.;
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
        let d = b * b - 4. * a * c;
        if d < 0. {
            vec![]
        } else {
            vec![
                Intersection::new((-b - d.sqrt()) / (2. * a), Box::new(*self)),
                Intersection::new((-b + d.sqrt()) / (2. * a), Box::new(*self)),
            ]
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Sphere;
    use crate::tuple::{Point3, Vector3};
    use crate::geo::{Ray, Intersectable};


    #[test]
    fn it_sets_the_object_on_the_intersection() {
        let r = Ray::new(Point3::point(0., 0., -5.), Vector3::vector(0., 0., 1.));
        let s = Sphere::unit();
        let xs = r.intersect(Box::new(s));
        assert_eq!(xs.count(), 2);
        if let Some(sphere) = xs[0].intersectable.as_any().downcast_ref() {
            assert_abs_diff_eq!(s, sphere);
        }
        if let Some(sphere) = xs[1].intersectable.as_any().downcast_ref() {
            assert_abs_diff_eq!(s, sphere);
        }
    }
}
