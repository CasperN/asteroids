
use sdl2::render::Canvas;
use sdl2::video::Window;

use controller::{Control, Controller};

mod momentum;
pub use self::momentum::*;


pub trait Controllable {
    fn control_update(&mut self, control: &Control);
}

pub trait Render {
    fn render(&self, canvas: &mut Canvas<Window>);
}

pub trait StateFrame {
    fn enter(&mut self, canvas: &mut Canvas<Window>, controller: &mut Controller);
}
