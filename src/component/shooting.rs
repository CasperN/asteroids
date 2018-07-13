use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub enum Projectile {
    Bullet,
    Asteroid,
}

#[derive(Debug)]
pub struct Shooting {
    pub is_firing: bool,
    fire_rate: Duration,
    last_fired: Instant,
    projectile: Projectile,
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
            projectile: Projectile::Bullet,
        }
    }
    pub fn new_asteroid_spawner() -> Shooting {
        let fire_rate = Duration::from_millis(1000);
        let last_fired = Instant::now() - fire_rate;
        Shooting {
            is_firing: true,
            fire_rate,
            last_fired,
            projectile: Projectile::Asteroid,
        }
    }
}
