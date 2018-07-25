extern crate rand;

use std::f32::consts::PI;

use component::{Momentum, Outline};
use entity::Entity;
use vector_2d::roots_of_unity;

#[derive(Debug)]
pub enum Shrapnel {
    Shards,
    Asteroids,
}

impl Shrapnel {
    pub fn shatter<R: rand::Rng>(
        &self,
        momentum: &Momentum,
        outline: &Outline,
        rng: &mut R,
    ) -> Vec<Entity> {

        let mut new_entities = Vec::new();

        match self {
            Shrapnel::Shards => {
                let points = outline.get_relative_outline();
                let n = points.len();
                for i in 0..n {
                    let a = points[i];
                    let b = points[(i + 1) % n];
                    let center = (a + b).scale(0.5);
                    let rotation = center.rotate(PI * 0.5).scale(momentum.omega);
                    let vel = center + rotation;

                    if rng.gen_range(0, 3) > 0 {
                        new_entities.push(
                            Entity::new()
                                .add_health(1)
                                .add_momentum(momentum.new_relative(center.scale(1.05), vel, 1.0))
                                .add_outline(Outline::new(
                                    vec![a - center, b - center],
                                    outline.color(),
                                )),
                        );
                    }
                }
            }
            Shrapnel::Asteroids => {
                let n_chunks = rng.gen_range(2, 5);
                // let old_radius = momentum.mass.sqrt();
                // let chunk_radius = old_radius / n_chunks as f32;
                // let chunk_mass = chunk_radius * chunk_radius;
                let chunk_mass = momentum.mass * 0.75 / n_chunks as f32;
                let chunk_radius = chunk_mass.sqrt();

                for dir in roots_of_unity(n_chunks).into_iter() {
                    let mut m = momentum.clone();
                    m.pos += dir.scale(chunk_radius * 1.25);
                    m.vel += dir.scale(1.5);
                    m.omega += rng.gen_range(-PI, PI);

                    new_entities.push(
                        Entity::new()
                            .add_health(chunk_mass as u32)
                            .add_outline(Outline::new_asteroid(rng, chunk_radius))
                            .add_momentum(m)
                            .add_shrapnel(Shrapnel::Shards),
                    );
                }
            }
        }
        new_entities
    }
}
