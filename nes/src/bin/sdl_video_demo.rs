extern crate nes;
extern crate sdl2;

use nes::io::video::{ChannelVideoOutput, VideoOutput};
use nes::ppu::palette::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::thread;

fn test_picture_shader(x: u8, y: u8) -> Color {
  match (x, y) {
    (x, y) if y < 180 && x < 36 => Color(255, 255, 255),
    (x, y) if y < 180 && x < 72 => Color(255, 255, 0),
    (x, y) if y < 180 && x < 108 => Color(0, 255, 255),
    (x, y) if y < 180 && x < 144 => Color(0, 255, 0),
    (x, y) if y < 180 && x < 180 => Color(255, 0, 255),
    (x, y) if y < 180 && x < 216 => Color(255, 0, 0),
    (x, y) if y < 180 && x < 255 => Color(0, 0, 255),

    (x, y) if y < 200 && x < 36 => Color(0, 0, 255),
    (x, y) if y < 200 && x < 72 => Color(0, 0, 0),
    (x, y) if y < 200 && x < 108 => Color(255, 0, 255),
    (x, y) if y < 200 && x < 144 => Color(0, 0, 0),
    (x, y) if y < 200 && x < 180 => Color(0, 255, 255),
    (x, y) if y < 200 && x < 216 => Color(0, 0, 0),
    (x, y) if y < 200 && x < 255 => Color(255, 255, 255),

    (x, y) if y < 220 => Color(x, x, x),
    (x, _) => {
      let v = (x / 20) * 20;
      Color(v, v, v)
    }
  }
}

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let mut event_pump = sdl_context.event_pump().unwrap();

  let window = video_subsystem
    .window("WeAreRust Nes", 512, 480)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();
  let texture_creator = canvas.texture_creator();
  let mut texture = texture_creator
    .create_texture_streaming(PixelFormatEnum::RGB24, 256, 240)
    .unwrap();

  let (mut video_output, receiver) = ChannelVideoOutput::new();

  // TODO: Start the PPU in this thread
  thread::spawn(move || loop {
    for y in 0..240 {
      for x in 0..255 {
        video_output.output_pixel(test_picture_shader(x, y));
      }
      video_output.horizontal_sync();
    }
    video_output.vertical_sync();
  });

  'running: loop {
    // Handle SDL events
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

    // Block until a video frame is received
    let frame = match receiver.recv() {
      Err(_) => break,
      Ok(f) => f,
    };

    // Write frame data to texture data
    texture
      .with_lock(None, |buffer: &mut [u8], pitch: usize| {
        frame.write_to_buffer(buffer, pitch);
      }).unwrap();

    // Draw the texture to the window
    let (width, height) = canvas.output_size().unwrap();
    canvas.clear();
    canvas
      .copy(
        &texture,
        Some(Rect::new(0, 0, 255, 240)),
        Some(Rect::new(0, 0, width, height)),
      ).unwrap();
    canvas.present();
  }
}
