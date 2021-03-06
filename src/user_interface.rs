use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

const TARGET_FPS: u64 = 60;

enum Controls {
    Up,
    Down,
    Left,
    Right,
    Pause,
    Shoot,
    Quit,
}

#[derive(Debug)]
pub struct UserInput {
    pub ud: i8,
    pub lr: i8,
    pub pause: bool,
    pub quit: bool,
    pub shoot: bool,
    pub update_time: Instant,
}

impl UserInput {
    pub fn elapsed_time(&self) -> f32 {
        let e = self.update_time.elapsed();
        let secs = e.as_secs() as f32;
        let millis = e.subsec_millis() as f32 / 1000.0;
        secs + millis
    }
}

pub struct UserInterface {
    pub user_input: UserInput,
    map: HashMap<Keycode, (Controls, bool)>,
    event_pump: sdl2::EventPump,
    next_frame: Instant,
}

impl UserInterface {
    pub fn new(sdl_context: &Sdl) -> Self {
        let mut map = HashMap::new();
        map.insert(Keycode::W, (Controls::Up, false));
        map.insert(Keycode::S, (Controls::Down, false));
        map.insert(Keycode::A, (Controls::Left, false));
        map.insert(Keycode::D, (Controls::Right, false));
        map.insert(Keycode::P, (Controls::Pause, false));
        map.insert(Keycode::Q, (Controls::Quit, false));
        map.insert(Keycode::Space, (Controls::Shoot, false));

        let user_input = UserInput {
            ud: 0,
            lr: 0,
            quit: false,
            pause: false,
            shoot: false,
            update_time: Instant::now(),
        };

        let event_pump = sdl_context.event_pump().unwrap();

        UserInterface {
            map,
            user_input,
            event_pump,
            next_frame: Instant::now() + Duration::from_millis(1000 / TARGET_FPS),
        }
    }

    pub fn sleep_until_next_frame(&mut self) {
        let now = Instant::now();
        if self.next_frame > now {
            thread::sleep(self.next_frame - now);
        }
        self.next_frame = now + Duration::from_millis(1000 / TARGET_FPS);
    }

    pub fn parse_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            if let Event::KeyDown {
                keycode: Some(key), ..
            } = event
            {
                if let Some((_, pressed)) = self.map.get_mut(&key) {
                    *pressed = true;
                }
            }
            if let Event::KeyUp {
                keycode: Some(key), ..
            } = event
            {
                if let Some((_, pressed)) = self.map.get_mut(&key) {
                    *pressed = false;
                }
            }
        }
        self.user_input.ud = 0;
        self.user_input.lr = 0;
        for (key, pressed) in self.map.values() {
            match key {
                Controls::Up => if *pressed {
                    self.user_input.ud += 1
                },
                Controls::Down => if *pressed {
                    self.user_input.ud -= 1
                },
                Controls::Left => if *pressed {
                    self.user_input.lr -= 1
                },
                Controls::Right => if *pressed {
                    self.user_input.lr += 1
                },
                Controls::Pause => self.user_input.pause = *pressed,
                Controls::Quit => self.user_input.quit = *pressed,
                Controls::Shoot => self.user_input.shoot = *pressed,
            }
        }
        self.user_input.update_time = Instant::now();
    }
}
