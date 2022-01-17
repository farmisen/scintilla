// use core::fmt::Debug;
use crate::geo::{Intersectable, Ray, Sphere};
use std::any::Any;
use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub intersectable: Intersectable,
}

impl Intersection {
    pub fn new(t: f64, intersectable: Intersectable) -> Self {
        Self {
            t: t,
            intersectable,
        }
    }
}

impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}:{})", self.t, self.intersectable)
    }
}

#[cfg(test)]
mod tests {
    use super::Intersection;
    use crate::geo::{Intersectable, Sphere};

    #[test]
    fn it_encapsulates_a_parameter_t_and_an_intersectable() {
        let s = Sphere::unit();
        let i = Intersection::new(3.5, Intersectable::Sphere(s));
        let Intersectable::Sphere(s1) = i.intersectable;
        assert_abs_diff_eq!(s, s1);
    }
}
