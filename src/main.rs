#![feature(duration_extras)]
#![feature(euclidean_division)]

extern crate sdl2;
// use std::boxed::Box;
use std::{thread, time};

mod collisions;
mod components;
mod controller;
mod entities;
mod game_over;
mod vector_2d;

use collisions::find_collisions;
use components::*;
use controller::Controller;

const X_LEN: f32 = 100.0;
const Y_LEN: f32 = 100.0;

fn pause_loop(_io: &mut Controller) {
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

        if io.user_input.quit {
            break 'game;
        }
        if io.user_input.pause {
            pause_loop(&mut io)
        }

        io.draw_background();

        ship.control_update(&io.user_input);
        ship.move_position(io.user_input.elapsed_time());
        ship.render(&mut io.canvas);

        projectiles = projectiles
            .into_iter()
            .map(|mut p| {
                p.move_position(io.user_input.elapsed_time());
                p.render(&mut io.canvas);
                p
            })
            .filter(|p| !p.should_despawn())
            .collect();

        if let Some(a) = asteroid_belt.maybe_shoot(&mut io.rng) {
            // projectiles.push(Box::new(a));
            projectiles.push(a);
        }
        if let Some(b) = ship.maybe_shoot(&mut io.rng) {
            // projectiles.push(Box::new(b));
            projectiles.push(b);
        }

        let mut outlines = Vec::new();
        outlines.push(ship.get_outline().0);
        for p in projectiles.iter() {
            outlines.push(p.get_outline().0);
        }
        let collisions = find_collisions(&outlines);
        for (a, b) in collisions {
            for x in [a, b].iter() {
                if *x > 0 {
                    projectiles[*x - 1].was_hit = true;
                } else {
                    ship.was_hit = true;
                }
            }
        }
        io.canvas.present();

        if ship.was_hit {
            game_over::gameover_loop(&mut io);
        }
    }
}
