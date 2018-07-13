#![feature(euclidean_division)]
#![feature(non_modrs_mods)]
// #![feature(alloc_system)]
// extern crate alloc_system;

use std::collections::{HashMap, HashSet};
use std::process;
use std::thread;
use std::time::{Duration, Instant};

extern crate sdl2;

mod component;
mod entity;
mod hud;
mod system;
mod user_interface;
mod vector_2d;

use entity::Entity;
use hud::Screen;
use user_interface::UserInterface;

const FONT_PATH: &'static str = "/Users/casperneo/Desktop/font.ttf";
const X_LEN: f32 = 100.0;
const Y_LEN: f32 = 100.0;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut screen = hud::Screen::new(&sdl_context, &ttf_context, FONT_PATH);
    let mut io = UserInterface::new(&sdl_context);

    let mut last_paused = Instant::now();
    let mut death_time = None;

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
        io.sleep_until_next_frame();

        if io.user_input.quit {
            break 'game;
        }
        if io.user_input.pause && last_paused.elapsed() > Duration::from_millis(300) {
            loop_text(&mut io, "Paused.", &mut screen, true);
            io.parse_events();
            last_paused = Instant::now();
        }
        screen.draw_background();

        let _ = system::control(&controls, &mut entities, &io);

        let out_of_bounds = system::move_position(&momentum, &mut entities, &mut io);
        let collisions = system::find_collisions(&outline, &mut entities);

        let _ = system::reflect(&collisions, &mut entities);
        let killed = system::damage(collisions, &mut entities);

        let projectiles = system::shoot(&shooting, &mut entities, &mut io);
        let shrapnel = system::shrapnel(&killed, &mut entities, &mut io.rng);

        for s in projectiles.into_iter().chain(shrapnel.into_iter()) {
            entities.insert(entity_num, s);
            outline.insert(entity_num);
            momentum.insert(entity_num);
            entity_num += 1;
        }

        for k in killed.into_iter().chain(out_of_bounds.into_iter()) {
            entities.remove(&k);
            momentum.remove(&k);
            outline.remove(&k);
        }

        system::render(&outline, &mut entities, &mut screen);

        // Render Health
        entities
            .get(&0)
            .and_then(|e| e.health)
            .map(|h| screen.draw_health(h));

        screen.canvas.present();

        if !entities.contains_key(&0) && death_time == None {
            death_time = Some(Instant::now());
        }
        if let Some(t) = death_time {
            if t.elapsed() > Duration::from_millis(5000) {
                loop_text(&mut io, "Game Over.", &mut screen, false);
            }
        }
    }
}

fn loop_text(io: &mut UserInterface, text: &str, screen: &mut Screen, escape: bool) {
    screen.draw_background();
    screen.draw_big_centered(text);
    screen.canvas.present();

    loop {
        thread::sleep(Duration::from_millis(100));
        io.parse_events();

        if io.user_input.quit {
            process::exit(0);
        }
        if io.user_input.pause && escape {
            io.parse_events();
            return;
        }
    }
}
