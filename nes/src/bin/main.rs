extern crate nes;
extern crate sdl2;

use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc;

use nes::apu::processor::ApuImpl;
use nes::console::Console;
use nes::controller::joypad;
use nes::io::audio::NesAudioProcess;
use nes::io::video;
use sdl2::audio::AudioSpecDesired;
use sdl2::keyboard::Keycode;

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

  // TODO(toby): parse the file content
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

  let _window = video_subsystem
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
    }).unwrap();

  apu_playback.resume();

  let (event_tx, event_rx) = mpsc::channel();
  let mut controller1 = joypad::Joypad::new(event_rx);
  let controller2: Option<&mut joypad::Joypad> = None;
  let (video_output, _receiver) = video::ChannelVideoOutput::new();

  let mut console = Console::new(
    &mut apu
    &mut cartridge,
    Some(&mut controller1),
    controller2,
    video_output,
  );

  console.reset();

  // Run the controller loop
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
          }).unwrap(),
        Event::KeyUp {
          keycode: Some(keycode),
          ..
        } => event_tx
          .send(ControllerEvent::ButtonUp {
            button: controller1_keymap(keycode),
          }).unwrap(),
        _ => {}
      };
    }

    console.tick();
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
