use std::path::Path;
use std::process;
use std::{thread, time};

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

use user_interface::UserInterface;

fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
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

    let cx = (UserInterface::SCREEN_WIDTH as i32 - w) / 2;
    let cy = (UserInterface::SCREEN_HEIGHT as i32 - h) / 2;
    Rect::new(cx as i32, cy as i32, w as u32, h as u32)
}

pub fn gameover_loop(io: &mut UserInterface) {
    io.draw_background();
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = Path::new("/Users/casperneo/Desktop/font.ttf");
    let texture_creator = io.canvas.texture_creator();
    let mut font = ttf_context.load_font(font_path, 128).unwrap();

    font.set_style(sdl2::ttf::STYLE_BOLD);

    let surface = font
        .render("Game Over!")
        .blended(Color::RGBA(255, 0, 0, 255))
        .unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let TextureQuery { width, height, .. } = texture.query();
    let padding = 64;
    let target = get_centered_rect(
        width,
        height,
        UserInterface::SCREEN_WIDTH - padding,
        UserInterface::SCREEN_HEIGHT - padding,
    );

    io.canvas.copy(&texture, None, Some(target)).unwrap();
    io.canvas.present();

    loop {
        io.parse_events();
        let twenty_millis = time::Duration::from_millis(20);
        thread::sleep(twenty_millis);

        if io.user_input.quit {
            process::exit(0);
        }
    }
}
