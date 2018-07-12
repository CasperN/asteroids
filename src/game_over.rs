use std::path::Path;
use std::process;
use std::{thread, time};

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

use user_interface::UserInterface;

const PADDING: u32 = 64;
static FONT_PATH: &'static str = "/Users/casperneo/Desktop/font.ttf";

fn get_centered_rect(
    rect_width: u32,
    rect_height: u32,
    screen_width: u32,
    screen_height: u32,
    padding: u32,
) -> Rect {
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

pub fn loop_text(io: &mut UserInterface, text: &str, escape: bool) {
    io.draw_background();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new(FONT_PATH);
    let texture_creator = io.canvas.texture_creator();
    let mut font = ttf_context.load_font(font_path, 128).unwrap();

    font.set_style(sdl2::ttf::STYLE_BOLD);

    let surface = font
        .render(text)
        .blended(Color::RGBA(255, 0, 0, 255))
        .unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let TextureQuery { width, height, .. } = texture.query();
    let frame = io.canvas.viewport();
    let screen_height = (frame.bottom() - frame.top()) as u32;
    let screen_width = (frame.right() - frame.left()) as u32;

    let target = get_centered_rect(width, height, screen_width, screen_height, PADDING);

    io.canvas.copy(&texture, None, Some(target)).unwrap();
    io.canvas.present();
    io.parse_events();

    loop {
        thread::sleep(time::Duration::from_millis(100));
        io.parse_events();

        if io.user_input.quit {
            process::exit(0);
        }
        if io.user_input.pause && escape {
            io.parse_events();
            return;
        }
    }
}
