use crate::tuple::Tuple;

pub type Point3 = Tuple;

impl Point3 {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn origin() -> Self {
        Self::point (0., 0., 0.)
    }
}
