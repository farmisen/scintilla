use std::f64::consts::PI;
use std::path::Path;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::geo::{Ray, Sphere};
use crate::matrix::Matrix4;
use crate::scene::PointLight;
use crate::tuple::{Point3, Vector3};

const CANVAS_SIZE: usize = 320;

pub fn run() {
    let ray_origin = Point3::point(0., 0., -10.);
    let wall_size = 7.;
    let pixel_size = wall_size / (CANVAS_SIZE as f64);
    let half = wall_size / 2.;
    let mut c = Canvas::new(CANVAS_SIZE, CANVAS_SIZE, Color::black());
    // let color = Color::red();
    let mut shape = Sphere::new(Point3::point(0., 0., 0.), 1.);
    shape.material.color = Color::new(1., 0.2, 1.);
    let light_position = Point3::point(-10., 10., -10.);
    let ligth_color = Color::new(1., 1., 1.);
    let light = PointLight::new(light_position, ligth_color);

    for y in 0..CANVAS_SIZE {
        let world_y = (-half + pixel_size * (y as f64)) as f64;
        for x in 0..CANVAS_SIZE {
            let world_x = (-half + pixel_size * (x as f64)) as f64;
            let position = Point3::point(world_x, world_y, 10.);
            let r = Ray::new(ray_origin, (position - ray_origin).normalized());
            let xs = shape.intersections(&r);
            if xs.count() > 0 {
                let hit = &xs[0];
                let position = r.position(hit.t);
                let normal = hit.intersectable.normal_at(position);
                let eye = -r.direction;
                let color = hit
                    .intersectable
                    .get_material()
                    .lighting(light, position, eye, normal);
                c.write_pixel(x, CANVAS_SIZE - y, color)
            }
        }
    }
    // println!("{}", c.to_ppm());
    c.save(Path::new("out/test.png"))
        .expect("Couldn’t save the png");
}
