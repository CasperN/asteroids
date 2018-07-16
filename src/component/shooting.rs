use std::time::{Duration, Instant};

extern crate rand;

use self::Projectile::*;
use component::*;
use entity::Entity;
use vector_2d::V2;

#[derive(Clone, Copy, Debug)]
pub enum Projectile {
    Bullet,
    Asteroid {
        speed_range: (f32, f32),
        mass_range: (f32, f32),
    },
}
impl Projectile {
    pub fn spawn_entity<R: rand::Rng>(
        &self,
        rng: &mut R,
        momentum: &Option<Box<Momentum>>,
    ) -> Entity {
        match self {
            Bullet => new_bullet(momentum.as_ref().unwrap()),
            Asteroid {
                speed_range,
                mass_range,
            } => new_asteroid(rng, speed_range, mass_range),
        }
    }
    pub fn level_up(&mut self) {
        if let Asteroid {
            mut speed_range,
            mut mass_range,
        } = self
        {
            speed_range.0 *= 1.1;
            speed_range.1 *= 1.1;
            mass_range.0 *= 1.1;
            mass_range.1 *= 1.1;
            *self = Asteroid {
                speed_range,
                mass_range,
            };
        } else {
            println!("Warning... tried to level up non asteroid shooter");
        }
    }
}

#[derive(Debug)]
pub struct Shooting {
    pub is_firing: bool,
    fire_rate: Duration,
    last_fired: Instant,
    pub projectile: Projectile,
}

impl Shooting {
    pub fn try_fire(&mut self) -> Option<Projectile> {
        if self.is_firing && self.last_fired.elapsed() > self.fire_rate {
            self.last_fired = Instant::now();
            return Some(self.projectile);
        }
        None
    }
    pub fn new_ship_gun() -> Shooting {
        let fire_rate = Duration::from_millis(300);
        let last_fired = Instant::now() - fire_rate;
        Shooting {
            is_firing: false,
            fire_rate,
            last_fired,
            projectile: Bullet,
        }
    }
    pub fn new_asteroid_spawner() -> Shooting {
        let fire_rate = Duration::from_millis(1000);
        let last_fired = Instant::now() - fire_rate;
        let speed_range = (5.0, 10.0);
        let mass_range = (50.0, 100.0);
        Shooting {
            is_firing: true,
            fire_rate,
            last_fired,
            projectile: Asteroid {
                speed_range,
                mass_range,
            },
        }
    }
}

fn new_asteroid<R: rand::Rng>(rng: &mut R, speeds: &(f32, f32), masses: &(f32, f32)) -> Entity {
    let radius = rng.gen_range(speeds.0, speeds.1);
    let speed = rng.gen_range(masses.0, masses.1) / radius;
    let mass = radius * radius;

    let e = Entity::new()
        .add_momentum(Momentum::new_random_edge(rng, speed, mass))
        .add_outline(Outline::new_asteroid(rng, radius))
        .add_health(radius as u32);

    if radius > 7.5 {
        e.add_shrapnel(Shrapnel::Asteroids)
    } else {
        e.add_shrapnel(Shrapnel::Shards)
    }
}

fn new_bullet(mc: &Momentum) -> Entity {
    Entity::new()
        .add_momentum(mc.new_relative(V2(0.0, 2.5), V2(0.0, 50.0), 1.0))
        .add_outline(Outline::new_bullet())
        .add_health(1)
}
