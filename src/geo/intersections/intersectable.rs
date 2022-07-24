// use core::fmt::Debug;
use std::fmt;
use std::any::Any;
use core::fmt::Debug;
use crate::{geo::{Ray, Intersections, Sphere}, tuple::{Point3, Vector3}, scene::Material};

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

    pub fn normal_at(&self, position: Point3) -> Vector3 {
        match self {
            Intersectable::Sphere(s) => {
                s.normal_at(position)
            }
        }
    }

    pub fn get_material(&self) -> Material {
        match self {
            Intersectable::Sphere(s) => {
                s.material
            }
        }
    }
}

impl fmt::Display for Intersectable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
