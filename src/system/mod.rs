use std::collections::{HashMap, HashSet};

// use component::*;
use entity::Entity;
use user_interface::UserInterface;

mod collision;
pub use self::collision::find_collisions;

type EMap = HashMap<usize, Entity>;

pub fn control(controllables: &[usize], entities: &mut EMap, io: &UserInterface) {
    for id in controllables.iter() {
        entities
            .get_mut(id)
            .and_then(|e| {
                if let Some(m) = e.momentum.as_mut() {
                    if let Some(s) = e.shooting.as_mut() {
                        if let Some(c) = e.control.as_mut() {
                            return Some((c, m, s));
                        }
                    }
                }
                None
            })
            .map(|(c, m, s)| {
                c.update(&io.user_input, m, s);
            });
    }
}

pub fn move_position(
    momentus: &HashSet<usize>,
    entities: &mut EMap,
    io: &UserInterface,
) -> Vec<usize> {
    let mut out_of_bounds = Vec::new();
    for id in momentus.iter() {
        let in_bounds = entities
            .get_mut(id)
            .and_then(|e| e.momentum.as_mut())
            .map_or(false, |m| {
                m.move_position(io.user_input.elapsed_time());
                m.in_bounds()
            });

        if !in_bounds {
            out_of_bounds.push(*id);
        }
    }
    out_of_bounds
}

pub fn shoot(shooters: &[usize], entities: &mut EMap, io: &mut UserInterface) -> Vec<Entity> {
    use component::Projectile::*;

    let mut new_entities = Vec::new();

    for id in shooters.iter() {
        let new_projectile = entities
            .get_mut(id)
            .and_then(|e| e.shooting.as_mut())
            .and_then(|s| s.try_fire());

        match new_projectile {
            Some(Asteroid) => new_entities.push(Entity::new_asteroid(&mut io.rng)),
            Some(Bullet) => {
                if let Some(m) = entities.get(id).and_then(|e| e.momentum.as_ref()) {
                    new_entities.push(Entity::new_bullet(&m))
                }
            }
            None => (),
        }
    }
    new_entities
}

pub fn render(outlines: &HashSet<usize>, entities: &mut EMap, io: &mut UserInterface) {
    for id in outlines.iter() {
        let ent = entities.get(id);
        let m = ent.and_then(|e| e.momentum.as_ref());
        let o = ent.and_then(|e| e.outline.as_ref());

        if let (Some(o), Some(m)) = (o, m) {
            o.render(&m, &mut io.canvas);
        }
    }
}

pub fn damage(
    collisions: Vec<(usize, usize)>,
    out_of_bounds: Vec<usize>,
    entities: &mut EMap,
) -> Vec<usize> {
    let mut dead_entities = Vec::new();
    let damaged = collisions
        .into_iter()
        .flat_map(|(a, b)| vec![a, b].into_iter())
        .chain(out_of_bounds.into_iter());

    for id in damaged {
        let dead = entities
            .get_mut(&id)
            .and_then(|e| e.health)
            .map_or(false, |mut h| {
                h = h.saturating_sub(1);
                h == 0
            });
        if dead {
            dead_entities.push(id);
        }
    }
    dead_entities
}
