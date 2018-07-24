use std::collections::HashMap;

extern crate rand;
use self::rand::Rng;

use collision;
use component::*;
use entity::Entity;
use hud::Screen;
use user_interface::UserInput;

type Cmap<T> = HashMap<usize, T>;

pub struct EntityManager {
    entity_num: usize,
    momentum: Cmap<Momentum>,
    health: Cmap<u32>,
    shooting: Cmap<Shooting>,
    control: Cmap<Control>,
    shrapnel: Cmap<Shrapnel>,
    outline: Cmap<Outline>,
}

impl EntityManager {
    fn new() -> Self {
        EntityManager {
            entity_num: 0,
            momentum: HashMap::new(),
            health: HashMap::new(),
            shooting: HashMap::new(),
            control: HashMap::new(),
            shrapnel: HashMap::new(),
            outline: HashMap::new(),
        }
    }

    pub fn new_with_ship_and_asteroid_spawner() -> Self {
        let mut em = Self::new();
        em.register(Entity::new_ship());
        em.register(Entity::new_asteroid_spawner());
        em
    }

    fn register(&mut self, e: Entity) {
        let Entity {
            momentum,
            outline,
            health,
            shooting,
            control,
            shrapnel,
        } = e;
        if let Some(c) = momentum {
            self.momentum.insert(self.entity_num, *c);
        }
        if let Some(c) = outline {
            self.outline.insert(self.entity_num, *c);
        }
        if let Some(c) = health {
            self.health.insert(self.entity_num, c);
        }
        if let Some(c) = shooting {
            self.shooting.insert(self.entity_num, *c);
        }
        if let Some(c) = control {
            self.control.insert(self.entity_num, *c);
        }
        if let Some(c) = shrapnel {
            self.shrapnel.insert(self.entity_num, c);
        }
        self.entity_num += 1;
    }
    fn remove(&mut self, id: &usize) {
        self.momentum.remove(id);
        self.outline.remove(id);
        self.health.remove(id);
        self.shooting.remove(id);
        self.control.remove(id);
        self.shrapnel.remove(id);
    }

    pub fn control_update(&mut self, usr: &UserInput) {
        for (id, c) in self.control.iter() {
            if let Some(m) = self.momentum.get_mut(id) {
                if let Some(s) = self.shooting.get_mut(id) {
                    c.update(usr, m, s);
                }
            }
        }
    }

    pub fn render(&mut self, screen: &mut Screen) {
        for (id, o) in self.outline.iter() {
            if let Some(m) = self.momentum.get(id) {
                o.render(m, &mut screen.canvas);
            }
        }
    }

    pub fn draw_health(&self, screen: &mut Screen) {
        self.health.get(&0).map(|h| screen.draw_health(*h));
    }

    pub fn ship_is_dead(&self) -> bool {
        !self.health.contains_key(&0)
    }

    pub fn level_up_asteroid_spawner(&mut self) {
        self.shooting.get_mut(&1).map(|s| s.projectile.level_up());
    }

    pub fn move_position(&mut self, usr: &UserInput) {
        let mut out_of_bounds = Vec::new();
        for (id, m) in self.momentum.iter_mut() {
            m.move_position(usr.elapsed_time());
            if !m.in_bounds() {
                out_of_bounds.push(*id);
            }
        }
        for id in out_of_bounds.iter() {
            self.remove(id);
        }
    }

    pub fn shoot<R: Rng>(&mut self, rng: &mut R) {
        let mut new_ents = Vec::new();
        for (id, s) in self.shooting.iter_mut() {
            if let Some(b) = s.try_fire() {
                new_ents
                    .push(b.spawn_entity(rng, &self.momentum.get(id).map(|m| Box::new(m.clone()))));
                // TODO make clean
            }
        }
        for e in new_ents.into_iter() {
            self.register(e);
        }
    }

    pub fn collide<R: Rng>(&mut self, rng: &mut R) {
        // Find collisions given absolute outlines
        let collisions = collision::grid_search(
            &self
                .outline
                .iter()
                .flat_map(|(id, o)| {
                    self.momentum
                        .get(id)
                        .map(|m| (*id, o.compute_outline(m)))
                        .into_iter()
                })
                .collect(),
        );
        // Reflect entities who collided
        for (id_a, id_b) in collisions.iter() {
            let force;
            if let (Some(a), Some(b)) = (self.momentum.get(id_a), self.momentum.get(id_b)) {
                let axis = (a.pos - b.pos).unit();
                // let momentum_into_collision = vb.dot(axis) * mb - va.dot(axis) * ma;
                force = Some(axis.scale(25.0 * (a.mass + b.mass)));
            } else {
                force = None;
            }
            if let Some(force) = force {
                self.momentum
                    .get_mut(id_a)
                    .map(|m| m.impart(force, 0.0, 1.0));
                self.momentum
                    .get_mut(id_b)
                    .map(|m| m.impart(-force, 0.0, 1.0));
            }
        }
        // Damage from collision
        let mut dead_ents = Vec::new();
        for id in collisions
            .into_iter()
            .flat_map(|(a, b)| vec![a, b].into_iter())
        {
            let dead = self.health.get_mut(&id).map_or(false, |h: &mut u32| {
                *h = h.saturating_sub(1);
                *h == 0
            });
            if dead {
                dead_ents.push(id);
            }
        }
        // break into shrapnel
        let mut new_ents = Vec::new();
        for id in dead_ents.iter() {
            let mc = self.momentum.get(id);
            let oc = self.outline.get(id);
            let sc = self.shrapnel.get(id);
            if let (Some(m), Some(o), Some(s)) = (mc, oc, sc) {
                for new_ent in s.shatter(m, o, rng) {
                    new_ents.push(new_ent);
                }
            }
        }
        // Remove dead entities and spawn new shrapnel
        for e in dead_ents.iter() {
            self.remove(e);
        }
        for e in new_ents.into_iter() {
            self.register(e);
        }
    }
}
