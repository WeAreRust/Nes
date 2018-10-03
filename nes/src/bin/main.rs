#![feature(duration_as_u128)]

extern crate nes;
extern crate sdl2;

use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use nes::apu::processor::ApuImpl;
use nes::console::Console;
use nes::controller::joypad;
use nes::io::audio::NesAudioProcess;
use nes::io::video;
use sdl2::audio::AudioSpecDesired;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    panic!("Must supply ROM filename.");
  }

  let filename = &args[1];
  println!("Loading ROM: {}", filename);
  let mut f = File::open(filename).expect("File not found");
  let mut data: Vec<u8> = vec![];
  f.read_to_end(&mut data).unwrap();

  println!("Initializing SDL2...");

  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();

  let audio_subsystem = sdl_context.audio().unwrap();
  let audio_spec_desired = AudioSpecDesired {
    freq: Some(48000),
    channels: Some(1),
    samples: Some(800),
  };

  let window = video_subsystem
    .window("WeAreRust Nes", 256, 240)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

  let (audio_tx, audio_rx) = mpsc::channel();
  let mut apu = ApuImpl::create(audio_tx);
  let apu_playback = audio_subsystem
    .open_playback(None, &audio_spec_desired, |spec| {
      NesAudioProcess::new(audio_rx, spec.freq as u32)
    })
    .unwrap();

  apu_playback.resume();

  let (event_tx, event_rx) = mpsc::channel();
  let mut controller1 = joypad::Joypad::new(event_rx);
  let controller2: Option<&mut joypad::Joypad> = None;
  let (video_output, vid_receiver) = video::ChannelVideoOutput::new();

  let mut canvas = window.into_canvas().build().unwrap();
  let texture_creator = canvas.texture_creator();
  let mut texture = texture_creator
    .create_texture_streaming(PixelFormatEnum::RGB24, 256, 240)
    .unwrap();

  // Spawn console thread
  thread::spawn(move || {
    let mut cartridge = nes::cartridge::parse_rom_file(&data).unwrap();
    // print!("PRG ROM DUMP");
    // for i in 0x8000..0xC000 {
    //   if (i - 0x8000) % 0x10 == 0 {
    //     println!();
    //     print!("${:04X}", i);
    //   }
    //   print!(" {:02x}", cartridge.mapper.read_addr(i));
    // }
    // println!();

    println!("Cartridge loaded.");

    let mut console = Console::new(
      &mut apu,
      &mut cartridge,
      Some(&mut controller1),
      controller2,
      video_output,
    );

    console.reset();

    const REPORT_RATE: u32 = 1_000_000;
    let mut report_throttle = Throttle::new(REPORT_RATE);
    let mut start = Instant::now();

    // Run the controller loop
    let mut ticks = 0u32;
    loop {
      console.tick();
      ticks += 1;
      if report_throttle.test() {
        let now = Instant::now();
        let span = now.duration_since(start);
        let clock_rate = ticks as f32 / span.as_millis() as f32;
        println!("{} Hertz", clock_rate * 1_000 as f32);
        ticks = 0;
        start = now;
      }
    }
  });

  'running: loop {
    for event in event_pump.poll_iter() {
      use joypad::ControllerEvent;
      use sdl2::event::Event;
      // We have to map SDL2 keyboard events to the correct
      // controller buttons.
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        Event::KeyDown {
          keycode: Some(keycode),
          ..
        } => event_tx
          .send(ControllerEvent::ButtonDown {
            button: controller1_keymap(keycode),
          })
          .unwrap(),
        Event::KeyUp {
          keycode: Some(keycode),
          ..
        } => event_tx
          .send(ControllerEvent::ButtonUp {
            button: controller1_keymap(keycode),
          })
          .unwrap(),
        _ => {}
      };
    }

    // Block until a video frame is received
    let frame = match vid_receiver.recv() {
      Err(_) => {
        println!("video feed disconnected");
        break 'running;
      }
      Ok(f) => f,
    };

    // Write frame data to texture data
    texture
      .with_lock(None, |buffer: &mut [u8], pitch: usize| {
        frame.write_to_buffer(buffer, pitch);
      })
      .unwrap();

    // Draw the texture to the window
    let (width, height) = canvas.output_size().unwrap();
    canvas.clear();
    canvas
      .copy(
        &texture,
        Some(Rect::new(0, 0, 255, 240)),
        Some(Rect::new(0, 0, width, height)),
      )
      .unwrap();
    canvas.present();
  }
}

fn controller1_keymap(keycode: Keycode) -> u8 {
  match keycode {
    Keycode::A => joypad::BUTTON_A,
    Keycode::B => joypad::BUTTON_B,
    Keycode::Return => joypad::BUTTON_START,
    Keycode::Space => joypad::BUTTON_SELECT,
    Keycode::Up => joypad::BUTTON_UP,
    Keycode::Down => joypad::BUTTON_DOWN,
    Keycode::Left => joypad::BUTTON_LEFT,
    Keycode::Right => joypad::BUTTON_RIGHT,
    _ => 0u8,
  }
}

struct Throttle {
  rate: u32,
  cursor: u32,
}

impl Throttle {
  pub fn new(rate: u32) -> Self {
    Self {
      rate: rate,
      cursor: 0,
    }
  }

  pub fn test(&mut self) -> bool {
    if self.cursor == self.rate {
      self.cursor = 0;
      true
    } else {
      self.cursor += 1;
      false
    }
  }
}
