use std::f32::consts::PI;

extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use controller::Control;
use traits::{Controllable, Render, Momentum, MomentumC};
use vector_2d::V2;

use X_LEN;
use Y_LEN;

pub struct Ship {
    momentum: MomentumC,
    torque: f32,
    thrust: f32,
}

impl Ship {
    const OUTLINE: [V2; 4] = [
        V2(-1.0, 0.0),
        V2(0.0, 2.0),
        V2(1.0, 0.0),
        V2(0.0, -1.0)
    ];

    pub fn new() -> Self {
        Ship {
            momentum: MomentumC {
                pos: V2(X_LEN / 2.0, Y_LEN / 2.0),
                theta: PI,
                vel: V2(0.0, 0.0),
                omega: 0.0,
                mass: 1.0,
            },
            torque: 10.0,
            thrust: 50.0
        }
    }

    fn get_outline(&self) -> Vec<V2> {
        let mc = self.get_momentum();
        Ship::OUTLINE
            .iter()
            .map(|p| p.rotate(mc.theta).add(mc.pos))
            .collect()
    }
}

impl Momentum for Ship {
    const SPEED_DECAY: f32 = 0.98;
    const WRAP_AROUND: bool = true;
    const ROTATION_DECAY: f32 = 0.95;

    fn get_momentum(&self) -> &MomentumC{
        &self.momentum
    }
    fn get_momentum_mut(&mut self) -> &mut MomentumC{
        &mut self.momentum
    }
}

impl Controllable for Ship {
    fn control_update(&mut self, control: &Control) {
        let torque = -control.lr as f32 * self.torque;
        let force = V2(0.0, self.thrust * control.ud as f32)
            .rotate(self.get_momentum().theta);
        let dt = control.elapsed_time();
        self.impart(force, torque, dt);
    }
}


impl Render for Ship {
    fn render(&self, canvas: &mut Canvas<Window>){
        let mut outline :Vec<V2> = self.get_outline();
        let head = outline[0];
        outline.push(head);

        let frame = canvas.viewport();
        let x_scale = (frame.right() - frame.left()) as f32 / X_LEN;
        let y_scale = (frame.bottom() - frame.top()) as f32 / Y_LEN;
        let offset = V2(frame.left() as f32, frame.top() as f32);

        let points = outline.iter()
            .map(|p| {
                 let V2(x, y) = p.scale_2d(x_scale, y_scale).add(offset);
                 Point::new(x as i32, y as i32)
            }).collect::<Vec<Point>>();


        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_lines(points.as_slice()).unwrap();
    }
}
