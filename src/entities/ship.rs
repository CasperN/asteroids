use std::f32::consts::PI;

extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use controller::Control;
use traits::{Position, Velocity, Controllable, Render};

use X_LEN;
use Y_LEN;

pub struct Ship {
    pos: (f32, f32),
    vel: (f32, f32),
    theta: f32,
    torque: f32,
    thrust: f32,
}

impl Ship {
    const OUTLINE: [(f32, f32); 4] = [(-1.0, 0.0), (0.0, 2.0), (1.0, 0.0), (0.0, -1.0)];

    pub fn new() -> Self {
        Ship {
            pos: (X_LEN / 2.0, Y_LEN / 2.0),
            vel: (0.0, 0.0),
            theta: PI,
            torque: 0.01,
            thrust: 0.0005
        }
    }

    fn get_outline(&self) -> Vec<(f32, f32)> {
        
        Ship::OUTLINE.iter().map( |(dx, dy)| {

            let (sin_th, cos_th) = self.theta.sin_cos();
            let x = dx * cos_th  + dy * sin_th;
            let y = dx * -sin_th + dy * cos_th;

            let (ship_x, ship_y) = self.pos;
            let x = ship_x + x;
            let y = ship_y + y;
            (x,y)

        }).collect()
    }
}


impl Position for Ship {
    fn get_xy(&self) -> (f32, f32) {
        self.pos
    }
    fn get_xy_mut(&mut self) -> (&mut f32, &mut f32){
        (&mut self.pos.0, &mut self.pos.1)
    }
}

impl Velocity for Ship {
    const WRAP_AROUND: bool = true;
    const SPEED_DECAY: f32 = 0.99;

    fn get_vxy(&self) -> (f32, f32) {
        self.vel
    }
    fn get_vxy_mut(&mut self) -> (&mut f32, &mut f32){
        (&mut self.vel.0, &mut self.vel.1)
    }
}

impl Controllable for Ship {
    fn control_update(&mut self, control: &Control) {
        let rotate = control.lr as f32;
        let thrust = control.ud as f32;
        let (sin_th, cos_th) = self.theta.sin_cos();
        let y_acc = thrust * cos_th * self.thrust;
        let x_acc = thrust * sin_th * self.thrust;

        let dt = control.elapsed_millis();

        self.theta = (self.theta + rotate * dt * self.torque).mod_euc(2.0 * PI);
        self.accelerate((x_acc, y_acc), dt);
    }
}


impl Render for Ship {
    fn render(&self, canvas: &mut Canvas<Window>){
        let mut outline :Vec<(f32, f32)> = self.get_outline();
        let head = outline[0];
        outline.push(head);

        let frame = canvas.viewport();
        let x_scale = (frame.right() - frame.left()) as f32 / X_LEN;
        let y_scale = (frame.top() - frame.bottom()) as f32 / Y_LEN;
        let x_off = frame.left() as f32;
        let y_off = frame.bottom() as f32;

        let points = outline.iter()
            .map( |(x,y)|
                Point::new((x * x_scale + x_off) as i32, (y * y_scale + y_off) as i32)
            ).collect::<Vec<Point>>();

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_lines(points.as_slice()).unwrap();
    }
}
