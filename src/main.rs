#![feature(duration_extras)]
#![feature(euclidean_division)]
extern crate sdl2;
use std::{thread, time};
// use sdl2::pixels::Color;

mod controller;
use controller::Controller;

mod traits;
use traits::{Render, Controllable, Velocity};

mod entities;

const X_LEN :f32 = 20.0;
const Y_LEN :f32 = 20.0;



fn pause_loop(io: &mut Controller){

}


fn main() {

    let mut io = Controller::new();
    let mut ship = entities::Ship::new();

    'game: loop {
        io.parse_events();
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);

        if io.user_input.quit { break 'game }
        if io.user_input.pause { pause_loop(&mut io) }
        io.draw_background();
        ship.render(&mut io.canvas);
        ship.control_update(&io.user_input);
        ship.move_position(io.user_input.elapsed_millis());


        io.canvas.present();
    }
}
