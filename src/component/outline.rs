extern crate rand;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use component::Momentum;
use vector_2d::{roots_of_unity, V2};
use X_LEN;
use Y_LEN;

pub struct Outline {
    relative_outline: Vec<V2>,
    color: Color,
}

impl Outline {
    pub fn compute_outline(&self, mc: &Momentum) -> Vec<V2> {
        self.relative_outline
            .iter()
            .map(|p| p.rotate(mc.theta) + mc.pos)
            .collect()
    }

    pub fn render(&self, momentum: &Momentum, canvas: &mut Canvas<Window>) {
        let mut outline = self.compute_outline(momentum);
        let o = outline[0];
        outline.push(o);

        let frame = canvas.viewport();
        let x_scale = (frame.right() - frame.left()) as f32 / X_LEN;
        let y_scale = (frame.bottom() - frame.top()) as f32 / Y_LEN;
        let offset = V2(frame.left() as f32, frame.top() as f32);

        let points: Vec<Point> = outline
            .iter()
            .map(|p| {
                let V2(x, y) = p.scale_2d(x_scale, y_scale) + offset;
                Point::new(x as i32, y as i32)
            })
            .collect();

        canvas.set_draw_color(self.color);
        canvas.draw_lines(points.as_slice()).unwrap();
    }

    pub fn new_ship() -> Outline {
        Outline {
            relative_outline: vec![V2(0.0, 2.0), V2(1.0, 0.0), V2(0.0, -1.0), V2(-1.0, 0.0)],
            color: Color::RGB(255, 0, 0),
        }
    }

    pub fn new_asteroid<R: rand::Rng>(rng: &mut R, size: f32) -> Outline {
        let relative_outline = roots_of_unity(rng.gen_range(5, 10))
            .iter()
            .map(|p| p.scale(size))
            .collect();
        Outline {
            relative_outline,
            color: Color::RGB(0, 255, 0),
        }
    }

    pub fn new_bullet() -> Outline {
        Outline {
            relative_outline: vec![V2(0.0, 1.0), V2(0.0, -1.0)],
            color: Color::RGB(255, 255, 255),
        }
    }
    pub fn new(relative_outline: Vec<V2>, color: Color) -> Outline {
        Outline {
            relative_outline,
            color,
        }
    }
}
