use std::collections::HashMap;
use std::time;

extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;

enum UserInput {
    Up, Down, Left, Right, Pause, Shoot, Quit
}

pub struct Control {
    pub ud: i8,
    pub lr: i8,
    pub pause: bool,
    pub quit: bool,
    pub shoot: bool,
    pub update_time: time::Instant
}

impl Control {
    pub fn elapsed_millis(&self) -> f32 {
        let e = self.update_time.elapsed();
        let secs = e.as_secs() as f32 * 1000.0;
        let millis = e.subsec_millis() as f32;
        secs + millis
    }
}

pub struct Controller {
    pub canvas: Canvas<Window>,
    pub user_input: Control,
    map: HashMap<Keycode, UserInput>,
    event_pump: sdl2::EventPump,

}

impl Controller {

    const BACKGROUND_COLOR: Color = Color { r:0, g:0, b:0, a:255, };

    pub fn new() -> Self {

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 800)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let mut map = HashMap::new();
        map.insert(Keycode::W, UserInput::Up);
        map.insert(Keycode::S, UserInput::Down);
        map.insert(Keycode::A, UserInput::Left);
        map.insert(Keycode::D, UserInput::Right);
        map.insert(Keycode::P, UserInput::Pause);
        map.insert(Keycode::Q, UserInput::Quit);
        map.insert(Keycode::Space, UserInput::Shoot);

        let user_input = Control {
            ud: 0,
            lr: 0,
            quit: false,
            pause: false,
            shoot: false,
            update_time: time::Instant::now()
        };

        let event_pump = sdl_context.event_pump().unwrap();

        Controller { map, canvas, user_input, event_pump }
    }

    pub fn parse_events(&mut self) {
        let mut ud = 0;
        let mut lr = 0;
        let mut pause = false;
        let mut quit = false;
        let mut shoot = false;

        for event in self.event_pump.poll_iter() {
            if let Event::KeyDown{keycode: Some(key), ..} = event {
                match self.map.get(&key) {
                    Some(UserInput::Up)    => if ud == 0 {ud = 1},
                    Some(UserInput::Down)  => if ud == 0 {ud = -1},
                    Some(UserInput::Left)  => if lr == 0 {lr = -1},
                    Some(UserInput::Right) => if lr == 0 {lr = 1},
                    Some(UserInput::Pause) => pause = true,
                    Some(UserInput::Shoot) => shoot = true,
                    Some(UserInput::Quit)  => quit = true,
                    None => (),
                }
            }
        }
        self.user_input = Control {ud, lr, pause, quit, shoot, update_time: time::Instant::now()}
    }

    pub fn draw_background(&mut self) {

        self.canvas.set_draw_color(Self::BACKGROUND_COLOR);
        self.canvas.clear();
    }
}
