use approx::AbsDiffEq;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Mul<Tuple> for Tuple {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl AbsDiffEq for Tuple {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        f64::abs_diff_eq(&self.x, &other.x, epsilon)
            && f64::abs_diff_eq(&self.y, &other.y, epsilon)
            && f64::abs_diff_eq(&self.z, &other.z, epsilon)
            && f64::abs_diff_eq(&self.w, &other.w, epsilon)
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::Tuple;

    #[test]
    fn it_is_a_point_when_w_equals_to_one() {
        let thing = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert!(thing.is_point());
        assert_abs_diff_eq!(thing.x, 4.3);
        assert_abs_diff_eq!(thing.y, -4.2);
        assert_abs_diff_eq!(thing.z, 3.1);
        assert_eq!(thing.is_vector(), false)
    }

    #[test]
    fn it_is_a_vector_when_w_equals_to_zero() {
        let thing = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert!(thing.is_vector());
        assert_abs_diff_eq!(thing.x, 4.3);
        assert_abs_diff_eq!(thing.y, -4.2);
        assert_abs_diff_eq!(thing.z, 3.1);
        assert_eq!(thing.is_point(), false)
    }

    #[test]
    fn it_creates_points() {
        let point = Tuple::point(4.3, -4.2, 3.1);
        assert_abs_diff_eq!(point.x, 4.3);
        assert_abs_diff_eq!(point.y, -4.2);
        assert_abs_diff_eq!(point.z, 3.1);
        assert_abs_diff_eq!(point.w, 1.0);
        assert!(point.is_point())
    }

    #[test]
    fn it_creates_vectors() {
        let vector = Tuple::vector(4.3, -4.2, 3.1);
        assert_abs_diff_eq!(vector.x, 4.3);
        assert_abs_diff_eq!(vector.y, -4.2);
        assert_abs_diff_eq!(vector.z, 3.1);
        assert_abs_diff_eq!(vector.w, 0.0);
        assert!(vector.is_vector())
    }

    #[test]
    fn it_adds_two_tuples() {
        let tuple1 = Tuple {
            x: 3.,
            y: -2.,
            z: 5.,
            w: 1.,
        };
        let tuple2 = Tuple {
            x: -2.,
            y: 3.,
            z: 1.,
            w: 0.,
        };
        assert_abs_diff_eq!(
            tuple1 + tuple2,
            Tuple {
                x: 1.,
                y: 1.,
                z: 6.,
                w: 1.
            }
        )
    }


    #[test]
    fn it_substracts_two_points() {
        let point1 = Tuple::point(3., 2., 1.);
        let point2 = Tuple::point(5., 6., 7.);
        assert_abs_diff_eq!(point1 - point2, Tuple::vector(-2., -4., -6.))
    }

   

    #[test]
    fn it_negates_a_tuple() {
        let tuple = Tuple {
            x: 1.,
            y: -2.,
            z: 3.,
            w: -4.,
        };
        assert_abs_diff_eq!(
            -tuple,
            Tuple {
                x: -1.,
                y: 2.,
                z: -3.,
                w: 4.
            }
        )
    }

    #[test]
    fn it_multiplies_a_tuple_by_a_scalar() {
        let tuple = Tuple {
            x: 1.,
            y: -2.,
            z: 3.,
            w: -4.,
        };
        assert_abs_diff_eq!(
            tuple * 3.5,
            Tuple {
                x: 3.5,
                y: -7.,
                z: 10.5,
                w: -14.
            }
        )
    }

    #[test]
    fn it_divides_a_tuple_by_a_scalar() {
        let tuple = Tuple {
            x: 1.,
            y: -2.,
            z: 3.,
            w: -4.,
        };
        assert_abs_diff_eq!(
            tuple / 2.,
            Tuple {
                x: 0.5,
                y: -1.,
                z: 1.5,
                w: -2.
            }
        )
    }

   
}
