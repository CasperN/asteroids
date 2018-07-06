
extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
// use sdl2::rect::Point;

use controller::{Control, Controller};
use X_LEN;
use Y_LEN;

pub trait Position {
    fn get_xy(& self) -> (f32, f32);
    fn get_xy_mut(&mut self) -> (&mut f32, &mut f32);
}

pub trait Velocity: Position {
    const SPEED_DECAY: f32 = 1.0;
    const WRAP_AROUND: bool;

    fn get_vxy (&self) -> (f32, f32);
    fn get_vxy_mut (&mut self) -> (&mut f32, &mut f32);

    fn move_position(&mut self, time: f32) {
        let (vx, vy) = self.get_vxy();
        let (x, y) = self.get_xy_mut();

        *x += vx * time;
        *y += vy * time;

        if Self::WRAP_AROUND {
            *x = x.mod_euc(X_LEN);
            *y = y.mod_euc(Y_LEN);
        }
    }

    fn accelerate(&mut self, axy: (f32, f32), time: f32) {
        let (ax, ay) = axy;
        let (vx, vy) = self.get_vxy_mut();
        *vx = (*vx + ax * time) * Self::SPEED_DECAY;
        *vy = (*vy + ay * time) * Self::SPEED_DECAY;
    }
}

pub trait Controllable {
    fn control_update(&mut self, control: &Control);
}

pub trait Render {
    fn render(&self, canvas: &mut Canvas<Window>);
}

pub trait StateFrame {
    fn enter(&mut self, canvas: &mut Canvas<Window>, controller: &mut Controller);
}
