#![feature(euclidean_division)]

use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::{Duration, Instant};

extern crate sdl2;

mod component;
mod entity;
mod game_over;
mod system;
mod user_interface;
mod vector_2d;

use entity::Entity;
use user_interface::UserInterface;

const X_LEN: f32 = 100.0;
const Y_LEN: f32 = 100.0;

fn main() {
    let twenty_millis = Duration::from_millis(20);
    let half_sec = Duration::from_millis(300);
    let mut last_paused = Instant::now();

    let mut io = UserInterface::new();
    let mut entities = HashMap::new();

    entities.insert(0, Entity::new_ship());
    entities.insert(1, Entity::new_asteroid_spawner());
    let controls = [0];
    let shooting = [0, 1];
    let mut momentum = HashSet::new();
    let mut outline = HashSet::new();
    momentum.insert(0);
    outline.insert(0);

    let mut entity_num = 2;

    'game: loop {
        io.parse_events();

        thread::sleep(twenty_millis);
        if io.user_input.quit {
            break 'game;
        }
        if io.user_input.pause && last_paused.elapsed() > half_sec {
            game_over::loop_text(&mut io, "Paused.", true);
            io.parse_events();
            last_paused = Instant::now();
        }
        io.draw_background();

        // Update systems... TODO give parts of IO, not the whole controller
        let _ = system::control(&controls, &mut entities, &io);
        let out_of_bounds = system::move_position(&momentum, &mut entities, &mut io);
        let collisions = system::find_collisions(&outline, &mut entities);
        let _ = system::reflect(&collisions, &mut entities);
        let killed = system::damage(collisions, out_of_bounds, &mut entities);
        let spawned = system::shoot(&shooting, &mut entities, &mut io);

        for s in spawned.into_iter() {
            entities.insert(entity_num, s);
            outline.insert(entity_num);
            momentum.insert(entity_num);
            entity_num += 1;
        }

        for k in killed.iter() {
            entities.remove(&k);
            momentum.remove(&k);
            outline.remove(&k);
        }

        system::render(&outline, &mut entities, &mut io);
        io.canvas.present();

        if !entities.contains_key(&0) {
            game_over::loop_text(&mut io, "Game Over.", false);
        }
    }
}
