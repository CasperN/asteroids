use std::time::{Duration, Instant};

extern crate rand;

use entities::Projectile;
use components::{Shooter,};


pub struct AsteroidSpawner {
    level: u32,
    fire_rate: Duration,
    last_fired: Instant,
}

impl AsteroidSpawner {
    pub fn new() -> Self {
        let fire_rate = Duration::from_millis(1000);
        let last_fired = Instant::now() - fire_rate;
        AsteroidSpawner {
            level: 1,
            fire_rate,
            last_fired,
        }
    }
}

impl Shooter for AsteroidSpawner {
    fn maybe_shoot<R:rand::Rng>(&mut self, rng: &mut R) -> Option<Projectile> {
        if self.last_fired.elapsed() < self.fire_rate {
            return None;
        }
        self.last_fired = Instant::now();

        let size = rng.gen_range(self.level as f32, self.level as f32 * 10.0);
        let speed = self.level as f32* 5.0;
        let asteroid = Projectile::new_asteroid(rng, size, speed);
        Some(asteroid)
    }
}
