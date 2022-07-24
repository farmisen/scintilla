use crate::{color::Color, tuple::Point3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointLight {
    pub position: Point3,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point3, intensity: Color) -> Self {
        Self { position, intensity }
    }
}

#[cfg(test)]
mod tests {
    use super::PointLight;
    use crate::color::Color;
    use crate::tuple::Point3;

    #[test]
    fn it_has_a_position_and_intensity() {
        let position = Point3::point(0., 0., 0.);
        let intensity = Color::new(1., 1., 1.);
        let light = PointLight::new(position, intensity);
        
        assert_abs_diff_eq!(light.position, position);
        assert_abs_diff_eq!(light.intensity, intensity);
        
    }
}