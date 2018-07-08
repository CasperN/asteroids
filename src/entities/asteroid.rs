use std::f32::consts::PI;

extern crate rand;

extern crate sdl2;
use sdl2::pixels::Color;

use traits::{Momentum, MomentumC, Render};
use vector_2d::{V2, roots_of_unity};
use X_LEN;
use Y_LEN;

#[derive(Debug)]
pub struct Asteroid {
    momentum: MomentumC,
    spikes: Vec<V2>,
}


impl Asteroid {
    pub fn new<R:rand::Rng>(rng: &mut R, speed: f32) -> Self {
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

        let momentum = MomentumC { pos, vel, theta, omega: 0.0, mass: 1.0 };

        let spikes = roots_of_unity(rng.gen_range(5, 10))
             .iter()
             .map(|p| p.scale(3.0))
             .collect();

        Asteroid { momentum, spikes }
    }
}


impl Momentum for Asteroid {
    const SPEED_DECAY: f32 = 0.0;
    const WRAP_AROUND: bool = true;
    const ROTATION_DECAY: f32 = 0.0;

    fn get_momentum(&self) -> &MomentumC{
        &self.momentum
    }
    fn get_momentum_mut(&mut self) -> &mut MomentumC{
        &mut self.momentum
    }
}


impl Render for Asteroid {
    fn get_outline(&self) -> Vec<V2> {
        let pos = self.get_momentum().pos;
        self.spikes.iter().map(|v| v.add(pos)).collect()
    }
    fn get_color(&self) -> Color {
        Color::RGB(0, 255, 0)
    }
}
