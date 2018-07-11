extern crate rand;

use std::f32::consts::PI;

use vector_2d::V2;
use X_LEN;
use Y_LEN;

#[allow(dead_code)]
pub enum EdgeBehaviour {
    Pass,
    PacMan,
    Wall,
}

use self::EdgeBehaviour::*;

pub struct Momentum {
    pub pos: V2,
    pub vel: V2,
    pub theta: f32,
    pub omega: f32,
    pub mass: f32, // also moment of inertia
    speed_decay: f32,
    rotation_decay: f32,
    edge_behavior: EdgeBehaviour,
}

impl Momentum {
    pub fn move_position(&mut self, time: f32) {
        self.pos = self.pos + self.vel.scale(time);
        match self.edge_behavior {
            PacMan => self.pos = self.pos.mod_euc(X_LEN, Y_LEN),
            Pass => (),
            Wall => {
                if self.pos.0 < 0.0 {
                    self.pos.0 = 0.0;
                    self.vel.0 = 0.0;
                }
                if self.pos.1 < 0.0 {
                    self.pos.1 = 0.0;
                    self.vel.1 = 0.0;
                }
                if self.pos.0 > X_LEN {
                    self.pos.0 = X_LEN;
                    self.vel.0 = 0.0;
                }
                if self.pos.1 > Y_LEN {
                    self.pos.1 = Y_LEN;
                    self.vel.1 = 0.0;
                }
            }
        }

        self.theta += self.omega * time;
        self.theta = self.theta.mod_euc(2.0 * PI);
    }

    pub fn impart(&mut self, force: V2, torque: f32, time: f32) {
        self.vel += force.scale(time / self.mass);
        self.vel = self.vel.scale(self.speed_decay.powf(time));

        self.omega += torque * time / self.mass;
        self.omega *= self.rotation_decay.powf(time);
    }

    pub fn in_bounds(&self) -> bool {
        let V2(x, y) = self.pos;
        0.0 <= x && x <= X_LEN && 0.0 <= y && y <= Y_LEN
    }

    pub fn new_centered() -> Momentum {
        Momentum {
            pos: V2(X_LEN / 2.0, Y_LEN / 2.0),
            theta: PI,
            vel: V2(0.0, 0.0),
            omega: 0.0,
            mass: 1.0,
            edge_behavior: Wall,
            speed_decay: 0.5,
            rotation_decay: 0.2,
        }
    }

    pub fn new_random_edge<R: rand::Rng>(rng: &mut R, speed: f32) -> Momentum {
        let theta = rng.gen_range(0.0, 2.0 * PI);
        let vel = V2(0.0, speed).rotate(theta);
        let pos = if vel.0.abs() < vel.1.abs() {
            V2(
                rng.gen_range(0.0, X_LEN),
                if vel.1 > 0.0 { 0.0 } else { Y_LEN },
            )
        } else {
            V2(
                if vel.0 > 0.0 { 0.0 } else { X_LEN },
                rng.gen_range(0.0, Y_LEN),
            )
        };

        Momentum {
            pos,
            vel,
            theta,
            omega: 0.0,
            mass: 1.0,
            edge_behavior: Pass,
            rotation_decay: 1.0,
            speed_decay: 1.0,
        }
    }

    pub fn new_relative(&self, pos: V2, vel: V2, mass: f32) -> Momentum {
        Momentum {
            pos: self.pos + pos.rotate(self.theta),
            vel: self.vel + vel.rotate(self.theta),
            theta: self.theta,
            omega: 0.0,
            mass,
            edge_behavior: Pass,
            rotation_decay: 1.0,
            speed_decay: 1.0,
        }
    }
}
