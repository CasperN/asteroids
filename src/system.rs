use std::collections::{HashMap, HashSet};

extern crate rand;

use entity::Entity;
use user_interface::UserInterface;
use hud::Screen;

mod collision;
pub use self::collision::find_collisions;

type EMap = HashMap<usize, Entity>;

pub fn control(controllables: &[usize], entities: &mut EMap, io: &UserInterface) {
    for id in controllables.iter() {
        entities.get_mut(id).map(|e| {
            if let Some(m) = e.momentum.as_mut() {
                if let Some(s) = e.shooting.as_mut() {
                    if let Some(c) = e.control.as_mut() {
                        c.update(&io.user_input, m, s);
                    }
                }
            }
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

pub fn render(outlines: &HashSet<usize>, entities: &mut EMap, screen: &mut Screen) {
    for id in outlines.iter() {
        let ent = entities.get(id);
        let m = ent.and_then(|e| e.momentum.as_ref());
        let o = ent.and_then(|e| e.outline.as_ref());

        if let (Some(o), Some(m)) = (o, m) {
            o.render(&m, &mut screen.canvas);
        }
    }
}

pub fn damage(collisions: Vec<(usize, usize)>, entities: &mut EMap) -> Vec<usize> {
    let mut dead_entities = Vec::new();
    let damaged = collisions
        .into_iter()
        .flat_map(|(a, b)| vec![a, b].into_iter());

    for id in damaged {
        let dead = entities.get_mut(&id).map_or(false, |e| {
            if let Some(mut h) = e.health {
                h = h.saturating_sub(1);
                e.health = Some(h);
                return h == 0;
            }
            false
        });
        if dead {
            dead_entities.push(id);
        }
    }
    dead_entities
}

pub fn reflect(collisions: &Vec<(usize, usize)>, entities: &mut EMap) {
    let get_pos_mass = |ents: &EMap, id| {
        ents.get(id)
            .and_then(|e| e.momentum.as_ref())
            .map(|m| (m.pos, m.mass, m.vel))
    };

    let impart = |ents: &mut EMap, id, force| {
        ents.get_mut(id)
            .and_then(|e| e.momentum.as_mut().map(|m| m.impart(force, 0.0, 1.0)))
    };

    for (id_a, id_b) in collisions.iter() {
        let mut a = get_pos_mass(entities, id_a);
        let mut b = get_pos_mass(entities, id_b);

        if let (Some((pa, ma, _va)), Some((pb, mb, _vb))) = (a, b) {
            // TODO consider velocity to soften collisions
            let axis = (pa - pb).unit();
            // let momentum_into_collision = vb.dot(axis) * mb - va.dot(axis) * ma;

            let fa = axis.scale(25.0 * (ma + mb)); //* momentum_into_collision);
            let fb = -fa;
            impart(entities, id_a, fa);
            impart(entities, id_b, fb);
        }
    }
}

pub fn shrapnel<R: rand::Rng>(
    killed: &Vec<usize>,
    entities: &mut EMap,
    rng: &mut R,
) -> Vec<Entity> {
    let mut new_entities = Vec::new();
    for id in killed.iter() {
        let ents = entities.get(id).and_then(|e| {
            let mc = e.momentum.as_ref();
            let oc = e.outline.as_ref();
            let sc = e.shrapnel.as_ref();
            if let (Some(m), Some(o), Some(s)) = (mc, oc, sc) {
                return Some(s.shatter(m, o, rng));
            }
            None
        });
        if let Some(mut es) = ents {
            new_entities.append(&mut es);
        }
    }
    new_entities
}
