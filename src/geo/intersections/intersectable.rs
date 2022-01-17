// use core::fmt::Debug;
use std::fmt;
use std::any::Any;
use core::fmt::Debug;
use crate::geo::{Ray, Intersections, Sphere};

#[derive(Debug, PartialEq)]
pub enum Intersectable  { // <>
    Sphere(Sphere),
}

impl Intersectable {
    pub fn intersections(&self, ray: &Ray) -> Intersections {
        match self {
            Intersectable::Sphere(s) => {
                s.intersections(ray)
            }
        }
    }
}

impl fmt::Display for Intersectable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
