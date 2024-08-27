use std::env;

use chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

mod chip8;

const SCALE: u32 = 15;
const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

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
        .window("chip8", WIDTH * SCALE, HEIGHT * SCALE)
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
            let x = i as u32 % WIDTH;
            let y = i as u32 / WIDTH;

            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}
