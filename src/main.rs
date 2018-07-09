#![feature(duration_extras)]
#![feature(euclidean_division)]
extern crate sdl2;
use std::{thread, time};

mod vector_2d;

mod controller;
use controller::Controller;

#[macro_use]
mod traits;
use traits::*;
mod entities;

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
            .filter(|p| p.in_bounds())
            .collect();

        if let Some(a) = asteroid_belt.maybe_shoot(&mut io.rng) {
            projectiles.push(a);
        }
        if let Some(b) = ship.maybe_shoot(&mut io.rng) {
            projectiles.push(b);
        }

        io.canvas.present();
    }
}
