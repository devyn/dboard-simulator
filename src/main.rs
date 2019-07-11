use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rwops::RWops;
use sdl2::rect::Rect;

use std::cmp::max;

mod errors;
use errors::*;

mod font;
use font::Font;

mod board;
use board::Board;

static LIGHT: &[u8] = include_bytes!("../res/light.bmp");

fn main() -> Result<()> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let pitch  = 16i32;
    let width  = 128i32;
    let height = 32i32;
 
    let window = video_subsystem.window("dboard simulator",
                                        (width*pitch) as u32,
                                        (height*pitch) as u32)
        .position_centered()
        .build()?;
 
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()?;
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let surface = Surface::load_bmp_rw(&mut RWops::from_bytes(&LIGHT)?)?;

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_from_surface(surface)?;

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut board = Board::new(width, height);

    let font = Font::new();

    font.render_str(&mut board, 0, 0, (255, 255, 255), "NH115");
    font.render_str(&mut board, 8*6, 0, (255, 0, 255), "Shunki =");

    font.render_str(&mut board, 0, 10, (255, 255, 255), "NH116");
    font.render_str(&mut board, 8*6, 10, (255, 160, 0), "Delayed @");

    font.render_str(&mut board, 0, 20, (255, 255, 255), "NH117");
    font.render_str(&mut board, 8*6, 20, (255, 0, 0), "On time 11:40");

    let mut i = 0u64;

    'running: loop {
        canvas.clear();

        for y in 0..height {
            for x in 0..width {
                let (r, g, b) = board.get(x, y).unwrap();
                //println!("{}, {} -> {}, {}, {}", x, y, r, g, b);
                let red   = max(32, r);
                let green = max(32, g);
                let blue  = max(32, b);
                texture.set_color_mod(red, green, blue);
                canvas.copy(&texture, None, Rect::new(pitch*x, pitch*y, pitch as u32, pitch as u32))?;
            }
        }

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        i += 1;

        if i % 5 == 0 {
            board.scroll_down(1);
        }
    }

    Ok(())
}
