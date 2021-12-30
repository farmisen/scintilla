use std::f64::consts::PI;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub fn run() {
    let mut c = Canvas::new(512, 512, Color::black());

    for a in 0..12 {
        let matrix = Matrix::identity(4)
            .rotate_z(PI / 6. * a as f64)
            .translate(255., 255., 0.);
        let point = Tuple::point(200., 0., 0.);
        let hour = matrix * point;
        c.write_pixel(hour.x as usize,  511 - hour.y as usize, Color::white())
    }

    println!("{}", c.to_ppm());
}
