#![feature(euclidean_division)]
#![feature(non_modrs_mods)]
// #![feature(alloc_system)]
// extern crate alloc_system;

use std::process;
use std::thread;
use std::time::{Duration, Instant};

extern crate rand;
extern crate sdl2;

mod collision;
mod component;
mod entity;
mod entity_manager;
mod hud;
mod user_interface;
mod vector_2d;

use entity_manager::EntityManager;
use hud::Screen;
use user_interface::UserInterface;

const FONT_PATH: &'static str = "src/font.ttf";
const X_LEN: f32 = 100.0;
const Y_LEN: f32 = 100.0;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut screen = hud::Screen::new(&sdl_context, &ttf_context, FONT_PATH);
    let mut io = UserInterface::new(&sdl_context);
    let mut next_level = Instant::now() + Duration::from_secs(10);
    let mut last_paused = Instant::now();
    let mut death_time = None;
    let mut level = 1;
    let mut entities = EntityManager::new_with_ship_and_asteroid_spawner();
    let mut rng = rand::thread_rng();

    'game: loop {
        io.parse_events();
        io.sleep_until_next_frame();

        if io.user_input.quit {
            break 'game;
        }
        if io.user_input.pause && last_paused.elapsed() > Duration::from_millis(300) {
            loop_text(&mut screen, &mut io, "Paused.", true);
            io.parse_events();
            last_paused = Instant::now();
        }
        screen.draw_background();

        entities.control_update(&mut io.user_input);
        entities.shoot(&mut rng);
        entities.collide(&mut rng);
        entities.move_position(&mut io.user_input);
        entities.render(&mut screen);
        entities.draw_health(&mut screen);
        screen.draw_level(level);

        screen.canvas.present();

        if entities.ship_is_dead() && death_time == None {
            death_time = Some(Instant::now());
        }
        if let Some(t) = death_time {
            if t.elapsed() > Duration::from_millis(5000) {
                loop_text(&mut screen, &mut io, "Game Over.", false);
            }
        }

        if Instant::now() > next_level {
            next_level += Duration::from_secs(10);
            level += 1;
            entities.level_up_asteroid_spawner();
        }
    }
}

fn loop_text(screen: &mut Screen, io: &mut UserInterface, text: &str, escape: bool) {
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
