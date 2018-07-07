use std::f32::consts::PI;

extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use controller::Control;
use traits::{Position, Velocity, Controllable, Render, AngularVelocity, Angle};

use X_LEN;
use Y_LEN;

pub struct Ship {
    pos: (f32, f32),
    vel: (f32, f32),
    theta: f32,
    omega: f32,
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
            torque: 10.0,
            omega: 0.0,
            thrust: 50.0
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

impl_Position!(Ship);
impl_Angle!(Ship);

impl Velocity for Ship {
    const WRAP_AROUND: bool = true;
    const SPEED_DECAY: f32 = 0.98;

    fn get_vel(&self) -> (f32, f32) {
        self.vel
    }
    fn get_vel_mut(&mut self) -> (&mut f32, &mut f32){
        (&mut self.vel.0, &mut self.vel.1)
    }
}

impl AngularVelocity for Ship {
    const ROTATION_DECAY: f32 = 0.95;

    fn get_omega(&self) -> f32 {
        self.omega
    }
    fn get_omega_mut(&mut self) -> &mut f32 {
        &mut self.omega
    }
}



impl Controllable for Ship {
    fn control_update(&mut self, control: &Control) {
        let rotate = control.lr as f32 * self.torque;
        let thrust = control.ud as f32;
        let (sin_th, cos_th) = self.theta.sin_cos();
        let y_acc = thrust * cos_th * self.thrust;
        let x_acc = thrust * sin_th * self.thrust;

        let dt = control.elapsed_time();

        self.angular_accelerate(rotate, dt);
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
