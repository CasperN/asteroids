
extern crate sdl2;
use sdl2::rect::Point;

pub struct Segment {
    left: Point;
    right: Point;
}

impl Segment {
    fn new(p1: Point, ) -> Self {

        let (x0, y0) = xy0;
        let (x1, y1) = xy1;
        if x0 < x1 {
            Segment{ left_x: x0, left_y: y0, right_x: x1, right_y: y1}
        } else {
            Segment{ left_x: x1, left_y: y1, right_x: x0, right_y: y0}
        }
    }

    fn slope(&self) -> f32 {
        let rise = self.right_y - self.left_y;
        let run = self.right_x - self.left_x;
        rise / run
    }

    fn intersection(&self, other: Segment) -> Option<(f32, f32)> {
        if self.left_x > other.right_x || self.right_x < other.left_x {
            return None;
        }

        unimplemented!();

    }
}
