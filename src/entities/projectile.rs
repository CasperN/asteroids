use std::f32::consts::PI;

extern crate rand;

extern crate sdl2;
use sdl2::pixels::Color;

use traits::{Momentum, Inertia, Outlinable};
use vector_2d::{V2, roots_of_unity};
use X_LEN;
use Y_LEN;

#[derive(Debug)]
pub struct Projectile {
    pub momentum: Inertia,
    pub relative_outline: Vec<V2>,
    pub color: Color,
}


impl Momentum for Projectile {
    const SPEED_DECAY: f32 = 0.0;
    const ROTATION_DECAY: f32 = 0.0;
    const WRAP_AROUND: bool = false;

    fn get_momentum(&self) -> &Inertia{
        &self.momentum
    }
    fn get_momentum_mut(&mut self) -> &mut Inertia{
        &mut self.momentum
    }
}


impl Outlinable for Projectile {
    fn get_outline(&self) -> (Vec<V2>, Color) {
        let pos = self.get_momentum().pos;
        let theta = self.get_momentum().theta;

        let outline = self.relative_outline.iter()
            .map(|v| v.rotate(theta).add(pos) )
            .collect();
        (outline, self.color)
    }
}

impl Projectile {

    pub fn new_asteroid<R: rand::Rng>(rng: &mut R, size: f32, speed: f32) -> Projectile {
        let theta = rng.gen_range(0.0, 2.0 * PI);
        let vel = V2(0.0, speed).rotate(theta);
        let pos = if vel.0.abs() < vel.1.abs() {
            V2 (
                rng.gen_range(0.0, X_LEN),
                if vel.1 > 0.0 { 0.0 } else { Y_LEN }
            )
        } else {
            V2 (
                if vel.0 > 0.0 { 0.0 } else { X_LEN },
                rng.gen_range(0.0, Y_LEN)
            )
        };

        let momentum = Inertia { pos, vel, theta, omega: 0.0, mass: 1.0 };

        let relative_outline = roots_of_unity(rng.gen_range(5, 10))
             .iter()
             .map(|p| p.scale(size))
             .collect();

        let color = Color::RGB(0, 255, 0);

        Projectile { momentum, relative_outline, color }
    }

    pub fn new_bullet<R: rand::Rng>(_rng: &mut R, source: &Inertia) -> Projectile {
        let momentum = Inertia {
            pos: source.pos,
            vel: source.vel.add(V2(0.0, 100.0).rotate(source.theta)),
            theta: source.theta,
            omega: 0.0,
            mass: 0.1,
        };
        let relative_outline = vec![V2(0.0, 0.0), V2(0.0, -1.0)];
        let color = Color::RGB(255, 255, 255);
        Projectile { momentum, relative_outline, color, }
    }

}
