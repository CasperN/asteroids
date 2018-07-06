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


#[derive(Debug)]
pub struct Control {
    pub ud: i8,
    pub lr: i8,
    pub pause: bool,
    pub quit: bool,
    pub shoot: bool,
    pub update_time: time::Instant
}

impl Control {
    pub fn elapsed_time(&self) -> f32 {
        let e = self.update_time.elapsed();
        let secs = e.as_secs() as f32;
        let millis = e.subsec_millis() as f32 / 1000.0;
        secs + millis
    }

    fn update(&mut self, ui: &UserInput, keydown: bool ) {
        match (ui, keydown) {
            (UserInput::Up,    true)  | (UserInput::Down, false)  => self.ud += 1,
            (UserInput::Up,    false) | (UserInput::Down, true)   => self.ud -= 1,

            (UserInput::Right, true)  | (UserInput::Left, false)  => self.lr += 1,
            (UserInput::Right, false) | (UserInput::Left, true)   => self.lr -= 1,

            (UserInput::Pause, down) => self.pause = down,
            (UserInput::Shoot, down) => self.shoot = down,
            (UserInput::Quit,  down) => self.quit = down
        }
    }
}

pub struct Controller {
    pub canvas: Canvas<Window>,
    pub user_input: Control,
    map: HashMap<Keycode, (UserInput, bool)>,
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
        map.insert(Keycode::W,      (UserInput::Up,    false));
        map.insert(Keycode::S,      (UserInput::Down,  false));
        map.insert(Keycode::A,      (UserInput::Left,  false));
        map.insert(Keycode::D,      (UserInput::Right, false));
        map.insert(Keycode::P,      (UserInput::Pause, false));
        map.insert(Keycode::Q,      (UserInput::Quit,  false));
        map.insert(Keycode::Space,  (UserInput::Shoot, false));

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

        for event in self.event_pump.poll_iter() {
            if let Event::KeyDown{keycode: Some(key), ..} = event {
                if let Some((key, pressed)) = self.map.get_mut(&key) {
                    *pressed = true;
                }
            }
            if let Event::KeyUp{keycode: Some(key), ..} = event {
                if let Some((key, pressed)) = self.map.get_mut(&key) {
                    *pressed = false;
                }
            }
        }
        self.user_input.ud = 0;
        self.user_input.lr = 0;
        for (key, pressed) in self.map.values() {
            match key {
                UserInput::Up    => if *pressed { self.user_input.ud += 1 },
                UserInput::Down  => if *pressed { self.user_input.ud -= 1 },
                UserInput::Left  => if *pressed { self.user_input.lr -= 1 },
                UserInput::Right => if *pressed { self.user_input.lr += 1 },
                UserInput::Pause => self.user_input.pause = *pressed,
                UserInput::Quit  => self.user_input.quit  = *pressed,
                UserInput::Shoot => self.user_input.shoot = *pressed,
            }
        }

    }

    pub fn draw_background(&mut self) {

        self.canvas.set_draw_color(Self::BACKGROUND_COLOR);
        self.canvas.clear();
    }
}
