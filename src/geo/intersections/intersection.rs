// use core::fmt::Debug;
use std::fmt;
use std::any::Any;
use core::fmt::Debug;
use crate::geo::{Ray, Sphere};


enum Intersectable {
    Sphere(Sphere),
}

impl Intersectable {
    fn intersections(&self, ray: &Ray) -> Vec<Intersection> {
        self.intersections(ray)
    }
}

// pub trait Intersectable: Debug {
//     fn intersections(&self, ray: &Ray) -> Vec<Intersection>;
//     fn as_any(&self) -> &dyn Any;
// }

// #[derive(Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub intersectable: Box<dyn Intersectable>
}

impl Intersection {
    pub fn new(t: f64, intersectable: Box<dyn Intersectable>) -> Self {
        Self {
            t: t,
            intersectable
        }
    }
}

impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{:?})", self.t, self.intersectable)
    }
}

// impl core::cmp::PartialEq for Intersection {
//     fn eq(&self, other: &Self) -> bool {
//         f64::abs_diff_eq(self.t, other.t) &&
                    
//     }
// }

#[cfg(test)]
mod tests {
    use super::Intersection;
    use crate::geo::Sphere;

    #[test]
    fn it_encapsulates_a_parameter_t_and_an_intersectable() {
        let sphere = Sphere::unit();
        let intersection = Intersection::new(3.5, Box::new(sphere));
        if let Some(s) = intersection.intersectable.as_any().downcast_ref() {
            assert_abs_diff_eq!(sphere, s);
        }
    }
}