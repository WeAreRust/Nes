extern crate nes;
extern crate sdl2;

use std::sync::mpsc;
use std::thread;
use std::time;

use nes::controller::joypad;
use sdl2::keyboard::Keycode;

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

  let (event_tx, event_rx) = mpsc::channel();
  let mut controller = joypad::Joypad::new(event_rx);

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

    let mut printed = false;
    if controller.pressed(joypad::BUTTON_A) {
      print!("A ");
      printed = true;
    }
    if controller.pressed(joypad::BUTTON_B) {
      print!("B ");
      printed = true;
    }
    if controller.pressed(joypad::BUTTON_SELECT) {
      print!("SELECT ");
      printed = true;
    }
    if controller.pressed(joypad::BUTTON_START) {
      print!("START ");
      printed = true;
    }
    if controller.pressed(joypad::BUTTON_UP) {
      print!("UP ");
      printed = true;
    }
    if controller.pressed(joypad::BUTTON_DOWN) {
      print!("DOWN ");
      printed = true;
    }
    if controller.pressed(joypad::BUTTON_LEFT) {
      print!("LEFT ");
      printed = true;
    }
    if controller.pressed(joypad::BUTTON_RIGHT) {
      print!("RIGHT ");
      printed = true;
    }
    if printed {
      println!();
    }

    thread::sleep(time::Duration::from_millis(25));
  }
}
