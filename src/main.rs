#![feature(duration_extras)]
#![feature(euclidean_division)]
extern crate sdl2;
use std::{thread, time};
// use sdl2::pixels::Color;

mod vector_2d;
mod controller;
use controller::Controller;
// mod asteroid;
mod momentum;


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

    'game: loop {
        io.parse_events();
        let twenty_millis = time::Duration::from_millis(20);
        thread::sleep(twenty_millis);

        if io.user_input.quit { break 'game }
        if io.user_input.pause { pause_loop(&mut io) }
        io.draw_background();
        ship.render(&mut io.canvas);
        ship.control_update(&io.user_input);
        ship.move_position(io.user_input.elapsed_time());
        ship.rotate(io.user_input.elapsed_time());

        io.canvas.present();
    }
}
