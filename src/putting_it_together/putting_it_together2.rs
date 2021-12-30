use crate::tuple::Tuple;
use crate::putting_it_together::shared::{Environment, Projectile, tick};
use crate::canvas::Canvas;
use crate::color::Color;


pub fn run() {
    let mut p = Projectile {
        position: Tuple::point(0., 1., 0.),
        velocity: Tuple::point(1., 1.8, 0.).normalized() * 11.25,
    };
    let e = Environment {
        gravity: Tuple::point(0., -0.1, 0.),
        wind: Tuple::point(-0.01, 0., 0.).normalized(),
    };

    let mut c = Canvas::new(900, 550, Color::black());

    while p.position.y > 0. {
        p = tick(&e, p);
        c.write_pixel(p.position.x as usize,  549 - p.position.y as usize, Color::white())
    }
    println!("{}", c.to_ppm());
}