use std::env::{self};

use chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

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
        chip8.update_timers();

        for _ in 0..30 {
            if check_keyboard(&mut event_pump, &mut chip8) {
                break 'running;
            }
            chip8.tick();
        }
        if chip8.wait_int == 1 {
            chip8.wait_int = 2;
        }
        draw_screen(&chip8, &mut canvas)
    }
    Ok(())
}

pub fn check_keyboard(event_pump: &mut EventPump, chip8: &mut Chip8) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return true,
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                if let Some(k) = keys(key) {
                    chip8.set_keyboard(k, true);
                }
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                if let Some(k) = keys(key) {
                    chip8.set_keyboard(k, false);
                }
            }
            _ => (),
        }
    }
    false
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

fn keys(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Y => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
