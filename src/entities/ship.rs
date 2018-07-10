use std::f32::consts::PI;
use std::time::{Duration, Instant};

extern crate rand;

extern crate sdl2;
use sdl2::pixels::Color;

use controller::Control;
use entities::{Projectile};
use components::{Controllable, Outlinable, Momentum, Inertia, Shooter};
use vector_2d::V2;
use X_LEN;
use Y_LEN;

pub struct Ship {
    momentum: Inertia,
    torque: f32,
    thrust: f32,
    is_firing: bool,
    fire_rate: Duration,
    last_fired: Instant,
}

impl Ship {
    const OUTLINE: [V2; 4] = [
        V2(-1.0, 0.0),
        V2(0.0, 2.0),
        V2(1.0, 0.0),
        V2(0.0, -1.0)
    ];

    pub fn new() -> Self {
        let fire_rate = Duration::from_millis(300);
        let last_fired = Instant::now() - fire_rate;

        Ship {
            momentum: Inertia {
                pos: V2(X_LEN / 2.0, Y_LEN / 2.0),
                theta: PI,
                vel: V2(0.0, 0.0),
                omega: 0.0,
                mass: 1.0,
            },
            torque: 10.0,
            thrust: 25.0,
            is_firing: false,
            fire_rate,
            last_fired,
        }
    }
}

impl Momentum for Ship {
    const SPEED_DECAY: f32 = 0.55;
    const WRAP_AROUND: bool = true;
    const ROTATION_DECAY: f32 = 0.25;

    fn get_momentum(&self) -> &Inertia{
        &self.momentum
    }
    fn get_momentum_mut(&mut self) -> &mut Inertia{
        &mut self.momentum
    }
}

impl Controllable for Ship {
    fn control_update(&mut self, control: &Control) {
        self.is_firing = control.shoot;
        let torque = -control.lr as f32 * self.torque;
        let force = V2(0.0, self.thrust * control.ud as f32)
            .rotate(self.get_momentum().theta);

        let dt = control.elapsed_time();
        self.impart(force, torque, dt);

    }
}

impl Shooter for Ship {
    fn maybe_shoot<R: rand::Rng>(&mut self, rng: &mut R) -> Option<Projectile> {
        if self.last_fired.elapsed() < self.fire_rate || !self.is_firing {
            return None;
        }
        self.last_fired = Instant::now();
        let bullet = Projectile::new_bullet(rng, self.get_momentum());
        Some(bullet)
    }
}

impl Outlinable for Ship {
    fn get_outline(&self) -> (Vec<V2>, Color) {
        let mc = self.get_momentum();
        let outline: Vec<V2> = Ship::OUTLINE
            .iter()
            .map(|p| p.rotate(mc.theta) + mc.pos)
            .collect();
        let color = Color::RGB(255, 0, 0);
        (outline, color)
    }
}
