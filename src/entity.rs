use std::boxed::Box;

extern crate rand;

use component::*;
use vector_2d::V2;

pub struct Entity {
    pub momentum: Option<Box<Momentum>>,
    pub outline: Option<Box<Outline>>,
    pub health: Option<u32>,
    pub shooting: Option<Box<Shooting>>,
    pub control: Option<Box<Control>>,
}

impl Entity {
    fn new() -> Self {
        Entity {
            momentum: None,
            outline: None,
            health: None,
            shooting: None,
            control: None,
        }
    }
    fn add_momentum(mut self, m: Momentum) -> Self {
        self.momentum = Some(Box::new(m));
        self
    }
    fn add_outline(mut self, o: Outline) -> Self {
        self.outline = Some(Box::new(o));
        self
    }
    fn add_shooting(mut self, s: Shooting) -> Self {
        self.shooting = Some(Box::new(s));
        self
    }
    fn add_control(mut self, torque: f32, thrust: f32) -> Self {
        let c = Control { torque, thrust };
        self.control = Some(Box::new(c));
        self
    }
    fn add_health(mut self, h: u32) -> Self {
        self.health = Some(h);
        self
    }

    pub fn new_asteroid_spawner() -> Entity {
        Entity::new().add_shooting(Shooting::new_asteroid_spawner())
    }

    pub fn new_ship() -> Entity {
        Entity::new()
            .add_momentum(Momentum::new_centered())
            .add_outline(Outline::new_ship())
            .add_shooting(Shooting::new_ship_gun())
            .add_control(10.0, 25.0)
            .add_health(10)
    }
    pub fn new_asteroid<R: rand::Rng>(rng: &mut R) -> Entity {
        Entity::new()
            .add_momentum(Momentum::new_random_edge(rng, 10.0))
            .add_outline(Outline::new_asteroid(rng, 5.0))
            .add_health(1)
    }
    pub fn new_bullet(mc: &Momentum) -> Entity {
        Entity::new()
            .add_momentum(mc.new_relative(V2(0.0, 3.0), V2(0.0, 50.0), 1.0))
            .add_outline(Outline::new_bullet())
            .add_health(1)
    }
}
