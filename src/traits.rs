
use std::f32::consts::PI;

extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
// use sdl2::rect::Point;

use controller::{Control, Controller};
use X_LEN;
use Y_LEN;

pub trait Position {
    fn get_pos(& self) -> (f32, f32);
    fn get_pos_mut(&mut self) -> (&mut f32, &mut f32);
}

macro_rules! impl_Position {
    ($T:ident) => {
        impl Position for $T {
            fn get_pos(&self) -> (f32, f32) {
                self.pos
            }
            fn get_pos_mut(&mut self) -> (&mut f32, &mut f32){
                (&mut self.pos.0, &mut self.pos.1)
            }
        }
    }
}

pub trait Velocity: Position {
    const SPEED_DECAY: f32;
    const WRAP_AROUND: bool;

    fn get_vel (&self) -> (f32, f32);
    fn get_vel_mut (&mut self) -> (&mut f32, &mut f32);

    fn move_position(&mut self, time: f32) {
        let (vx, vy) = self.get_vel();
        let (x, y) = self.get_pos_mut();

        *x += vx * time;
        *y += vy * time;

        if Self::WRAP_AROUND {
            *x = x.mod_euc(X_LEN);
            *y = y.mod_euc(Y_LEN);
        }
    }

    fn accelerate(&mut self, axy: (f32, f32), time: f32) {
        let (ax, ay) = axy;
        let (vx, vy) = self.get_vel_mut();
        *vx = (*vx + ax * time) * Self::SPEED_DECAY;
        *vy = (*vy + ay * time) * Self::SPEED_DECAY;
    }
}

pub trait Angle {
    fn get_theta(&self) -> f32;
    fn get_theta_mut(&mut self) -> &mut f32;
}
macro_rules! impl_Angle {
    ($T:ident) => {
        impl Angle for $T {
            fn get_theta(&self) -> f32 {
                self.theta
            }
            fn get_theta_mut(&mut self) -> &mut f32 {
                &mut self.theta
            }
        }
    }
}


pub trait AngularVelocity: Angle {
    const ROTATION_DECAY: f32;

    fn get_omega(&self) -> f32;
    fn get_omega_mut(&mut self) -> &mut f32;

    fn angular_accelerate(&mut self, acc: f32, time: f32 ) {
        let omega = self.get_omega_mut();
        *omega = (*omega + acc * time) * Self::ROTATION_DECAY;
    }

    fn rotate(&mut self, time: f32) {
        let w = self.get_omega();
        let theta = self.get_theta_mut();
        *theta = (*theta + w * time).mod_euc(2.0 * PI);
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
