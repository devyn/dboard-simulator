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

mod yvr;
use yvr::Flight;

static LIGHT: &[u8] = include_bytes!("../res/light.bmp");

fn main() -> Result<()> {
    //let flights = yvr::get_flights()?;
    let flights = vec![
        Flight {
            flight_number: "AC2012".into(),
            flight_city: "Torino".into(),
            flight_status: "Cancelled".into(),
            ..Flight::default()
        },
        Flight {
            flight_number: "KL442".into(),
            flight_city: "Malta".into(),
            flight_status: "On Time".into(),
            ..Flight::default()
        },
    ];

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let pitch  = 16i32;
    let width  = 192i32;
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

    for (index, flight) in flights.iter().take(3).enumerate() {
        font.render_str(&mut board,
                        0, (index * 10) as i32,
                        (255, 255, 255),
                        &flight.flight_number);

        font.render_str(&mut board,
                        48, (index * 10) as i32,
                        (255, 255, 255),
                        &flight.flight_city);

        font.render_str(&mut board,
                        128, (index * 10) as i32,
                        (0, 255, 0),
                        &flight.flight_status);
    }

    let mut counter = 0u64;
    let mut yoff = 0i32;

    'running: loop {
        if counter % 5 == 0 {
            board.clear_all();

            for (index, flight) in flights.iter().enumerate() {
                render_flight(&font, &mut board, yoff, index, flight);
            }

            //yoff -= 1;
        }

        counter += 1;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

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
    }

    Ok(())
}

fn render_flight(font: &Font, board: &mut Board, yoff: i32, index: usize, flight: &Flight) {
    let y = (index * 10) as i32 + yoff;

    if y < -10 || y >= 60 {
        return;
    }

    font.render_str(board,
                    0, (index * 10) as i32 + yoff,
                    (255, 255, 255),
                    &flight.flight_number);

    font.render_str(board,
                    48, (index * 10) as i32 + yoff,
                    (255, 255, 160),
                    &flight.flight_city);

    let status_color =
        match &flight.flight_status[..] {
            "Delayed" => (255, 160, 0),
            "Cancelled" => (255, 0, 0),
            _ => (0, 255, 0),
        };

    font.render_str(board,
                    128, (index * 10) as i32 + yoff,
                    status_color,
                    &flight.flight_status);
}
