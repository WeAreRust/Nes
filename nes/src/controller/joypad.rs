use std::sync::mpsc;
use std::thread;
use std::time;

// Loop 100 times a second.
const LOOP_SLEEP: time::Duration = time::Duration::from_millis(10);

pub const BUTTON_A: u8 = 1u8;
pub const BUTTON_B: u8 = 2u8;
pub const BUTTON_START: u8 = 4u8;
pub const BUTTON_SELECT: u8 = 8u8;
pub const BUTTON_UP: u8 = 16u8;
pub const BUTTON_DOWN: u8 = 32u8;
pub const BUTTON_LEFT: u8 = 64u8;
pub const BUTTON_RIGHT: u8 = 128u8;

// We have to use our own keyboard event because sdl2::event::Event does not implement Send.
pub enum ControllerEvent {
  ButtonDown { button: u8 },
  ButtonUp { button: u8 },
}

pub struct Joypad {
  button_state: u8,
  rx: mpsc::Receiver<u8>,
}

impl Joypad {
  // Each Joypad controller takes a channel receiver which will be sent button events
  // as they are mapped by the caller.
  pub fn new(event_rx: mpsc::Receiver<ControllerEvent>) -> Self {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
      println!("Starting controller...");

      let mut button_state = 0u8;
      loop {
        let mut new_state = button_state;

        match event_rx.recv().unwrap() {
          ControllerEvent::ButtonDown { button, .. } => new_state |= button,
          ControllerEvent::ButtonUp { button, .. } => new_state ^= button,
        };

        if new_state != button_state {
          tx.send(new_state).unwrap();
          button_state = new_state;
        }
        thread::sleep(LOOP_SLEEP);
      }
    });

    Self {
      button_state: 0u8,
      rx: rx,
    }
  }

  pub fn cycle(&mut self) {
    match self.rx.try_recv() {
      Ok(state) => {
        self.button_state = state;
        // Consume all messages off the channel
        self.cycle();
      }
      Err(mpsc::TryRecvError::Empty) => (),
      Err(mpsc::TryRecvError::Disconnected) => panic!("Joypad disconnected."),
    };
  }

  pub fn pressed(&self, button: u8) -> bool {
    self.button_state & button > 0
  }
}
