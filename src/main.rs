#![feature(duration_extras)]
#![feature(euclidean_division)]

extern crate sdl2;
use std::{thread, time};

mod vector_2d;
mod collisions;
mod controller;
mod entities;
mod components;

use collisions::find_collisions;
use controller::Controller;
use components::*;

const X_LEN :f32 = 100.0;
const Y_LEN :f32 = 100.0;


fn pause_loop(_io: &mut Controller){
    unimplemented!();
}


fn main() {

    let mut io = Controller::new();
    let mut ship = entities::Ship::new();
    let mut projectiles = Vec::<entities::Projectile>::new();
    let mut asteroid_belt = entities::AsteroidSpawner::new();

    'game: loop {
        io.parse_events();
        let twenty_millis = time::Duration::from_millis(20);
        thread::sleep(twenty_millis);

        if io.user_input.quit { break 'game }
        if io.user_input.pause { pause_loop(&mut io) }

        io.draw_background();

        ship.control_update(&io.user_input);
        ship.move_position(io.user_input.elapsed_time());
        ship.render(&mut io.canvas);

        projectiles = projectiles.into_iter()
            .map(|mut p|{
                p.move_position(io.user_input.elapsed_time());
                p.render(&mut io.canvas);
                p
            })
            .filter(|p| !p.should_despawn())
            .collect();

        if let Some(a) = asteroid_belt.maybe_shoot(&mut io.rng) {
            projectiles.push(a);
        }
        if let Some(b) = ship.maybe_shoot(&mut io.rng) {
            projectiles.push(b);
        }

        let outlines = projectiles.iter().map(|p| p.get_outline().0).collect();
        let collisions = find_collisions(&outlines);
        for (a,b) in collisions {
            projectiles[a].was_hit = true;
            projectiles[b].was_hit = true;
        }
        io.canvas.present();
    }
}
