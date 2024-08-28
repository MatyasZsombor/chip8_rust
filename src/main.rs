use std::env::{self};

use chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

mod chip8;
mod consts;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Example usage: chip8 file.ch8")
    }

    let mut chip8 = Chip8::new();
    println!("{}", &args[1]);
    chip8.load_rom(&args[1]);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "chip8",
            (consts::WIDTH * consts::SCALE) as u32,
            (consts::HEIGHT * consts::SCALE) as u32,
        )
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("could not make a canvas");

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        chip8.tick();
        draw_screen(&chip8, &mut canvas)
    }
    Ok(())
}

pub fn draw_screen(chip8: &Chip8, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    canvas.clear();

    let buffer = chip8.get_screen();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for (i, pixel) in buffer.iter().enumerate() {
        if *pixel {
            let x = i % consts::WIDTH;
            let y = i / consts::WIDTH;

            let rect = Rect::new(
                (x * consts::SCALE) as i32,
                (y * consts::SCALE) as i32,
                consts::SCALE as u32,
                consts::SCALE as u32,
            );
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}
