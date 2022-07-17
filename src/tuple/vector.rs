use crate::tuple::Tuple;

pub type Vector3 = Tuple;

impl Vector3 {
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }
}