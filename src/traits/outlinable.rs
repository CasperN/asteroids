extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use vector_2d::V2;
use X_LEN;
use Y_LEN;


pub trait Outlinable {

    fn get_outline(&self) -> (Vec<V2>, Color);

    fn render(&self, canvas: &mut Canvas<Window>){
        let (mut outline, color) = self.get_outline();
        let head = outline[0];
        outline.push(head);

        let frame = canvas.viewport();
        let x_scale = (frame.right() - frame.left()) as f32 / X_LEN;
        let y_scale = (frame.bottom() - frame.top()) as f32 / Y_LEN;
        let offset = V2(frame.left() as f32, frame.top() as f32);

        let points: Vec<Point> = outline.iter()
            .map( |p| {
                 let V2(x, y) = p.scale_2d(x_scale, y_scale).add(offset);
                 Point::new(x as i32, y as i32)
            }).collect();

        canvas.set_draw_color(color);
        canvas.draw_lines(points.as_slice()).unwrap();
    }
}
