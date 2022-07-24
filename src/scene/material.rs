use crate::{
    color::Color,
    tuple::{Point3, Vector3},
};

use super::PointLight;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn default() -> Self {
        Self::new(Color::new(1., 1., 1.), 0.1, 0.9, 0.9, 200.)
    }

    pub fn lighting(
        &self,
        light: PointLight,
        position: Point3,
        eye_vector: Vector3,
        normal_vector: Vector3,
    ) -> Color {
        // combine the surface color with the light's color intensity
        let effective_color = self.color * light.intensity;

        // calculate the direction to the light source
        let light_vector = (light.position - position).normalized();

        // calculate the ambent contribution
        let ambient_contrib = effective_color * self.ambient;

        // light_dot_normal is the cosine of the angle between light and normal vectors
        // if negative then the light is on the other side of the surface
        let light_dot_normal = light_vector.dot(normal_vector);

        let (diffuse_contrib, specular_contrib) = if light_dot_normal < 0. {
            (Color::black(), Color::black())
        } else {
            // calculate the diffuse contribution
            let diffuse_contrib = effective_color * self.diffuse * light_dot_normal;

            let reflected_vector = (-light_vector).reflect(normal_vector);

            // reflect_dot_eye is the cosine of the angle between reflected and eye vectors
            // if negative then the light is reflecting away from the eye
            let reflect_dot_eye = reflected_vector.dot(eye_vector);

            let specular_contrib = if reflect_dot_eye < 0. {
                Color::black()
            } else {
                // calculate the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            };

            (diffuse_contrib, specular_contrib)
        };
        ambient_contrib + diffuse_contrib + specular_contrib
    }
}

#[cfg(test)]
mod tests {
    use super::Material;
    use crate::color::Color;
    use crate::scene::PointLight;
    use crate::tuple::{Point3, Vector3};

    #[test]
    fn it_has_default() {
        let material = Material::default();
        assert_abs_diff_eq!(material.color, Color::new(1., 1., 1.));
    }

    #[test]
    fn it_calculate_lighting_when_the_eye_is_between_the_light_and_the_surface() {
        let material = Material::default();
        let position = Point3::point(0., 0., 0.);
        let eye_vector = Vector3::vector(0., 0., -1.);
        let normal_vector = Vector3::vector(0., 0., -1.);
        let light = PointLight::new(Point3::point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = material.lighting(light, position, eye_vector, normal_vector);
        assert_abs_diff_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn it_calculate_lighting_when_the_eye_is_between_the_light_and_the_surface_and_the_eye_is_offseted_45_degree(
    ) {
        let material = Material::default();
        let position = Point3::point(0., 0., 0.);
        let eye_vector = Vector3::vector(0., f64::sqrt(2.) / 2., f64::sqrt(2.) / 2.);
        let normal_vector = Vector3::vector(0., 0., -1.);
        let light = PointLight::new(Point3::point(0., 0., -10.), Color::new(1., 1., 1.));
        let result = material.lighting(light, position, eye_vector, normal_vector);
        assert_abs_diff_eq!(result, Color::new(1., 1., 1.));
    }

    #[test]
    fn it_calculate_lighting_when_the_eye_is_on_the_opposite_surface_and_the_light_is_offseted_45_degree(
    ) {
        let material = Material::default();
        let position = Point3::point(0., 0., 0.);
        let eye_vector = Vector3::vector(0., 0., -1.);
        let normal_vector = Vector3::vector(0., 0., -1.);
        let light = PointLight::new(Point3::point(0., 10., -10.), Color::new(1., 1., 1.));
        let result = material.lighting(light, position, eye_vector, normal_vector);
        let expected_value = 0.1 + 0.9 * f64::sqrt(2.) / 2. + 0.;
        assert_abs_diff_eq!(
            result,
            Color::new(expected_value, expected_value, expected_value)
        );
    }

    #[test]
    fn it_calculate_lighting_when_the_eye_is_in_the_path_of_the_reflection_vector() {
        let material = Material::default();
        let position = Point3::point(0., 0., 0.);
        let eye_vector = Vector3::vector(0., -f64::sqrt(2.) / 2., -f64::sqrt(2.) / 2.);
        let normal_vector = Vector3::vector(0., 0., -1.);
        let light = PointLight::new(Point3::point(0., 10., -10.), Color::new(1., 1., 1.));
        let result = material.lighting(light, position, eye_vector, normal_vector);
        let expected_value = 0.1 + 0.9 * f64::sqrt(2.) / 2. + 0.9;
        assert_abs_diff_eq!(
            result,
            Color::new(expected_value, expected_value, expected_value)
        );
    }

    #[test]
    fn it_calculate_lighting_when_the_light_is_behind_the_surface() {
        let material = Material::default();
        let position = Point3::point(0., 0., 0.);
        let eye_vector = Vector3::vector(0., 0., -1.);
        let normal_vector = Vector3::vector(0., 0., -1.);
        let light = PointLight::new(Point3::point(0., 0., 10.), Color::new(1., 1., 1.));
        let result = material.lighting(light, position, eye_vector, normal_vector);
        assert_abs_diff_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
