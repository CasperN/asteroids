use std::boxed::Box;

extern crate rand;

use component::*;

#[derive(Debug)]
pub struct Entity {
    pub momentum: Option<Box<Momentum>>,
    pub outline: Option<Box<Outline>>,
    pub health: Option<u32>,
    pub shooting: Option<Box<Shooting>>,
    pub control: Option<Box<Control>>,
    pub shrapnel: Option<Shrapnel>,
}

impl Entity {
    pub fn new() -> Self {
        Entity {
            momentum: None,
            outline: None,
            health: None,
            shooting: None,
            control: None,
            shrapnel: None,
        }
    }
    pub fn add_momentum(mut self, m: Momentum) -> Self {
        self.momentum = Some(Box::new(m));
        self
    }
    pub fn add_outline(mut self, o: Outline) -> Self {
        self.outline = Some(Box::new(o));
        self
    }
    pub fn add_shooting(mut self, s: Shooting) -> Self {
        self.shooting = Some(Box::new(s));
        self
    }
    pub fn add_control(mut self, torque: f32, thrust: f32) -> Self {
        let c = Control { torque, thrust };
        self.control = Some(Box::new(c));
        self
    }
    pub fn add_health(mut self, h: u32) -> Self {
        self.health = Some(h);
        self
    }
    pub fn add_shrapnel(mut self, s: Shrapnel) -> Self {
        self.shrapnel = Some(s);
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
            .add_control(100.0, 250.0)
            .add_health(10)
            .add_shrapnel(Shrapnel::Shards)
    }
}
