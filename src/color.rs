use approx::{AbsDiffEq};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn black() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn white() -> Self {
        Self::new(1., 1., 1.)
    }

    pub fn red() -> Self {
        Self::new(1., 0., 0.)
    }

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    fn channel_to_ppm(channel: f64) -> String {
        format!("{}", (channel * 255.).ceil() as u8)
    }

    pub fn to_ppm(&self) -> [String; 3] {
        [
            Color::channel_to_ppm(self.r),
            Color::channel_to_ppm(self.g),
            Color::channel_to_ppm(self.b),
        ]
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl AbsDiffEq for Color {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        f64::abs_diff_eq(&self.r, &other.r, epsilon)
            && f64::abs_diff_eq(&self.g, &other.g, epsilon)
            && f64::abs_diff_eq(&self.b, &other.b, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn it_creates_colors() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert_abs_diff_eq!(color.r, -0.5);
        assert_abs_diff_eq!(color.g, 0.4);
        assert_abs_diff_eq!(color.b, 1.7);
    }

    #[test]
    fn it_adds_two_colors() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        assert_abs_diff_eq!(color1 + color2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn it_substracts_one_color_from_another() {
        let color1 = Color::new(0.9, 0.6, 0.75);
        let color2 = Color::new(0.7, 0.1, 0.25);
        assert_abs_diff_eq!(color1 - color2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn it_multiply_a_color_by_a_scaler() {
        let color = Color::new(0.2, 0.3, 0.4);
        assert_abs_diff_eq!(color * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn it_multiplies_two_colors() {
        let color1 = Color::new(1., 0.2, 0.4);
        let color2 = Color::new(0.9, 1., 0.1);
        assert_abs_diff_eq!(color1 * color2, Color::new(0.9, 0.2, 0.04));
    }
}
