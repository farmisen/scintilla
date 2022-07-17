use crate::color::Color;
use image::io::Reader;
use std::{io::Cursor, path::Path};

#[derive(Debug)]
pub struct Canvas {
    buffer: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: Color) -> Self {
        Self {
            width,
            height,
            buffer: vec![color; width * height],
        }
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> &Color {
        &self.buffer[y * self.width + x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < self.width && y < self.height);
        self.buffer[y * self.width + x] = color
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");
        let mut is_new_line = true;
        let mut current_line_lenght: usize = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.read_pixel(x, y);
                for ppm_channel in color.to_ppm() {
                    if current_line_lenght + ppm_channel.chars().count() + 1 > 70 {
                        ppm.push_str("\n");
                        current_line_lenght = 0;
                        is_new_line = true;
                    }
                    if !is_new_line {
                        ppm.push_str(" ");
                        current_line_lenght += 1;
                    }
                    ppm.push_str(&ppm_channel);
                    is_new_line = false;
                    current_line_lenght += ppm_channel.chars().count();
                }
            }
            ppm.push_str("\n");
            current_line_lenght = 0;
            is_new_line = true;
        }
        ppm
    }

    pub fn save(&self, path: &Path) -> Result<(), image::ImageError> {
        let data = Cursor::new(self.to_ppm());
        let reader = Reader::new(data)
            .with_guessed_format()
            .expect("This will never fail using Cursor");
        let img = reader.decode().expect("Failed to read image");
        img.save(path)
    }
}

#[cfg(test)]
mod tests {
    use super::Canvas;
    use crate::color::Color;
    use indoc::indoc;

    #[test]
    fn it_creates_canvases() {
        let canvas = Canvas::new(10, 20, Color::new(0., 0., 0.));
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.read_pixel(5, 10), &Color::new(0., 0., 0.));
    }

    #[test]
    fn it_write_pixels() {
        let mut canvas = Canvas::new(10, 20, Color::new(0., 0., 0.));
        let red = Color::new(1., 0., 0.);
        canvas.write_pixel(2, 3, red);
        assert_eq!(canvas.read_pixel(2, 3), &red);
    }

    #[test]
    fn it_constructs_the_ppm_header() {
        let canvas = Canvas::new(5, 3, Color::new(0., 0., 0.));
        let ppm = canvas.to_ppm();
        let expected = indoc! {"P3
                             5 3
                             255"};
        let lines = &(ppm.split("\n").collect::<Vec<&str>>())[..3].join("\n");
        assert_eq!(lines, expected);
        assert!(ppm.starts_with(expected));
    }

    #[test]
    fn it_constructs_the_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3, Color::new(0., 0., 0.));
        let color1 = Color::new(1.5, 0., 0.);
        let color2 = Color::new(0., 0.5, 0.);
        let color3 = Color::new(-0.5, 0., 1.);
        canvas.write_pixel(0, 0, color1);
        canvas.write_pixel(2, 1, color2);
        canvas.write_pixel(4, 2, color3);

        let ppm = canvas.to_ppm();

        let expected = indoc! {"255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
                                    0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
                                    0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"};
        let lines = &(ppm.split("\n").collect::<Vec<&str>>())[3..6].join("\n");
        assert_eq!(expected, lines);
    }

    #[test]
    fn it_splits_long_lines_in_the_ppm_pixel_data() {
        let canvas = Canvas::new(10, 2, Color::new(1., 0.8, 0.6));
        let ppm = canvas.to_ppm();
        let expected = indoc! {"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
                                    153 255 204 153 255 204 153 255 204 153 255 204 153
                                    255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
                                    153 255 204 153 255 204 153 255 204 153 255 204 153"};
        let lines = &(ppm.split("\n").collect::<Vec<&str>>())[3..7].join("\n");
        assert_eq!(expected, lines);
    }

    #[test]
    fn it_terminates_the_ppm_with_a_newline_char() {
        let canvas = Canvas::new(5, 3, Color::new(1., 0.8, 0.6));
        let ppm = canvas.to_ppm();
        assert!(ppm.ends_with("\n"));
    }
}
