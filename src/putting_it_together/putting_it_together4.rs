use std::f64::consts::PI;
use std::path::Path;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::geo::{Ray, Sphere};
use crate::matrix::Matrix4;
use crate::tuple::{Point3, Vector3};

const CANVAS_SIZE: usize = 100;

pub fn run() {
    let ray_origin = Point3::point(0., 0., -10.);
    let wall_size = 7.;
    let pixel_size = wall_size / (CANVAS_SIZE as f64);
    let half = wall_size / 2.;
    let mut c = Canvas::new(CANVAS_SIZE, CANVAS_SIZE, Color::black());
    let color = Color::red();
    let shape = Sphere::new(Point3::point(0., 0., 0.), 1.);

    for y in 0..CANVAS_SIZE {
        let world_y = (-half + pixel_size * (y as f64)) as f64;
        for x in 0..CANVAS_SIZE {
            let world_x = (-half + pixel_size * (x as f64)) as f64;
            let position = Point3::point(world_x, world_y, 10.);
            let r = Ray::new(ray_origin, (position - ray_origin).normalized());
            let xs = shape.intersections(&r);
            if xs.count() > 0 {
                c.write_pixel(x, y, color)
            }
        }
    }
    println!("{}", c.to_ppm());
    c.save(Path::new("out/test.png"));
}
