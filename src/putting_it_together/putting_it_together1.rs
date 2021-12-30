use crate::tuple::Tuple;
use crate::putting_it_together::shared::{Environment, Projectile, tick};

pub fn run() {
    let mut p = Projectile {
        position: Tuple::point(0., 1., 0.),
        velocity: Tuple::point(1., 1., 0.).normalized(),
    };
    let e = Environment {
        gravity: Tuple::point(0., -0.1, 0.),
        wind: Tuple::point(-0.01, 0., 0.).normalized(),
    };

    while p.position.y > 0. {
        p = tick(&e, p);
        println!("({},{})", p.position, p.velocity)
    }
}