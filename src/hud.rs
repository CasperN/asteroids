use std::path::Path;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureQuery;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::Sdl;

const PADDING: u32 = 64;

pub struct Screen<'a> {
    pub canvas: Canvas<Window>,
    font: Font<'a, 'static>,
    top_left: Rect,
}

impl<'a> Screen<'a> {
    pub const SCREEN_WIDTH: u32 = 800;
    pub const SCREEN_HEIGHT: u32 = 800;

    const BACKGROUND_COLOR: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub fn new(sdl_context: &Sdl, ttf_context: &'a Sdl2TtfContext, font_path: &str) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("rust-sdl2 demo", Self::SCREEN_WIDTH, Self::SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut font = ttf_context.load_font(Path::new(font_path), 128).unwrap();
        font.set_style(sdl2::ttf::STYLE_BOLD);

        Screen {
            font,
            top_left: Rect::new(10, 10, 100, 20),
            canvas: window.into_canvas().build().unwrap(),
        }
    }
    pub fn draw_background(&mut self) {
        self.canvas.set_draw_color(Self::BACKGROUND_COLOR);
        self.canvas.clear();
    }

    pub fn draw_health(&mut self, health: u32) {
        let texture_creator = self.canvas.texture_creator();
        let surface = self
            .font
            .render(&format!("health: {:?}", health))
            .blended(Color::RGB(255, 255, 0))
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        self.canvas
            .copy(&texture, None, Some(self.top_left))
            .unwrap();
    }

    pub fn draw_big_centered(&mut self, text: &str) {
        let surface = self
            .font
            .render(text)
            .blended(Color::RGB(255, 0, 0))
            .unwrap();
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let TextureQuery { width, height, .. } = texture.query();
        let target = get_centered_rect(width, height, self.canvas.viewport(), PADDING);
        self.canvas.copy(&texture, None, Some(target)).unwrap();
    }
}

fn get_centered_rect(rect_width: u32, rect_height: u32, frame: Rect, padding: u32) -> Rect {
    let (screen_width, screen_height) = frame.size() as (u32, u32);

    let cons_width = screen_width - padding;
    let cons_height = screen_height - padding;

    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (screen_width as i32 - w) / 2;
    let cy = (screen_height as i32 - h) / 2;
    Rect::new(cx as i32, cy as i32, w as u32, h as u32)
}
