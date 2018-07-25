extern crate nes;
extern crate sdl2;

use nes::io::video::{SdlVideoOutput, VideoOutput};
use nes::ppu::palette::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let window = video_subsystem
        .window("WeAreRust Nes", 256, 240)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut video_output = SdlVideoOutput::new(canvas, &texture_creator);

    let mut blue: u8 = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Draw pixels one by one...
        blue = (blue + 1) % 255;
        for x in 0..255 {
            for y in 0..240 {
                video_output.output_pixel(Color(x, y, blue));
            }
            video_output.horizontal_sync();
        }
        video_output.vertical_sync();
    }
}
