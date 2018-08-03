use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time;

use controller::Controller;
use memory::{ReadAddr, WriteAddr};

// Loop 100 times a second.
const LOOP_SLEEP: time::Duration = time::Duration::from_millis(10);

pub const BUTTON_A: u8 = 1u8;
pub const BUTTON_B: u8 = 2u8;
pub const BUTTON_SELECT: u8 = 4u8;
pub const BUTTON_START: u8 = 8u8;
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
  button_state: Arc<Mutex<u8>>,
  strobe_state: State,
}

enum State {
  Init,
  Strobe,
  ReportA,
  ReportB,
  ReportSelect,
  ReportStart,
  ReportUp,
  ReportDown,
  ReportLeft,
  ReportRight,
}

impl Joypad {
  // Each Joypad controller takes a channel receiver which will be sent button events
  // as they are mapped by the caller.
  pub fn new(event_rx: mpsc::Receiver<ControllerEvent>) -> Self {
    let state = Arc::new(Mutex::new(0x00));

    let thread_state = state.clone();
    thread::spawn(move || {
      println!("Starting controller...");

      'running: loop {
        match event_rx.recv() {
          Ok(ControllerEvent::ButtonDown { button, .. }) => {
            let mut new_state = thread_state.lock().unwrap();
            *new_state |= button
          }
          Ok(ControllerEvent::ButtonUp { button, .. }) => {
            let mut new_state = thread_state.lock().unwrap();
            *new_state ^= button
          }
          Err(mpsc::RecvError) => {
            println!("Controller channel disconnected. Ending loop.");
            break 'running;
          }
        };
      }
    });

    Self {
      button_state: state,
      strobe_state: State::Init,
    }
  }

  pub fn pressed(&self, button: u8) -> bool {
    let state = self.button_state.lock().unwrap();
    *state & button > 0
  }

  fn strobe(&mut self) {
    match self.strobe_state {
      State::Init => self.strobe_state = State::Strobe,
      _ => (),
    }
  }

  fn strobe_end(&mut self) {
    match self.strobe_state {
      State::Strobe => self.strobe_state = State::ReportA,
      _ => (),
    }
  }

  fn read_next(&mut self) -> u8 {
    let map_pressed = |is_pressed: bool| {
      if is_pressed {
        0x01
      } else {
        0x00
      }
    };

    let result = match self.strobe_state {
      State::Init => 0x00,
      State::Strobe | State::ReportA => map_pressed(self.pressed(BUTTON_A)),
      State::ReportB => map_pressed(self.pressed(BUTTON_B)),
      State::ReportSelect => map_pressed(self.pressed(BUTTON_SELECT)),
      State::ReportStart => map_pressed(self.pressed(BUTTON_START)),
      State::ReportUp => map_pressed(self.pressed(BUTTON_UP)),
      State::ReportDown => map_pressed(self.pressed(BUTTON_DOWN)),
      State::ReportLeft => map_pressed(self.pressed(BUTTON_LEFT)),
      State::ReportRight => map_pressed(self.pressed(BUTTON_RIGHT)),
    };
    self.strobe_state = Self::next_state(&self.strobe_state, false);
    result
  }

  fn next_state(current: &State, strobe: bool) -> State {
    match strobe {
      true => State::Strobe,
      false => match current {
        State::Init => State::Init,
        State::Strobe => State::ReportA,
        State::ReportA => State::ReportB,
        State::ReportB => State::ReportSelect,
        State::ReportSelect => State::ReportStart,
        State::ReportStart => State::ReportUp,
        State::ReportUp => State::ReportDown,
        State::ReportDown => State::ReportLeft,
        State::ReportLeft => State::ReportRight,
        State::ReportRight => State::Init,
      },
    }
  }
}

impl Controller for Joypad {}

impl ReadAddr for Joypad {
  fn read_addr(&mut self, _addr: u16) -> u8 {
    // A controller only has a single address ($4016 for Player 1 and $4017 for Player 2) so ignore.
    self.read_next()
  }
}

impl WriteAddr for Joypad {
  fn write_addr(&mut self, _addr: u16, value: u8) -> u8 {
    // A controller only has a single address ($4016 for Player 1 and $4017 for Player 2) so ignore.
    match value & 0x01 == 0x01 {
      true => self.strobe(),
      false => self.strobe_end(), // Ignore any other writes
    }

    // TODO(toby): I think this is supposed to be self.pressed(BUTTON_A) if we are strobing.
    0x00
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_cycle() {
    let (tx, rx) = mpsc::channel();
    let mut joypad = Joypad::new(rx);

    // Send buttons A, Select and Right
    tx.send(ControllerEvent::ButtonDown { button: BUTTON_A })
      .unwrap();
    tx.send(ControllerEvent::ButtonDown {
      button: BUTTON_SELECT,
    }).unwrap();
    tx.send(ControllerEvent::ButtonDown {
      button: BUTTON_RIGHT,
    }).unwrap();

    // An annoying hack to avoid a race condition
    // with event handling vs following reads.
    thread::sleep(time::Duration::from_millis(1));

    // Starts in init state so read should be 0x00
    assert_eq!(joypad.read_addr(0x4016), 0x00);

    // Strobe
    joypad.write_addr(0x4016, 0x01);
    joypad.write_addr(0x4016, 0x00);

    // First read should be Button A
    assert_eq!(joypad.read_addr(0x4016), 0x01);

    // Next read is B
    assert_eq!(joypad.read_addr(0x4016), 0x00);
    // Next read is Select
    assert_eq!(joypad.read_addr(0x4016), 0x01);
    // Next read is Start
    assert_eq!(joypad.read_addr(0x4016), 0x00);
    // Next read is Up
    assert_eq!(joypad.read_addr(0x4016), 0x00);
    // Next read is Down
    assert_eq!(joypad.read_addr(0x4016), 0x00);
    // Next read is Left
    assert_eq!(joypad.read_addr(0x4016), 0x00);
    // Next read is Right
    assert_eq!(joypad.read_addr(0x4016), 0x01);

    // Finally, we should be back to init state so 0x00
    assert_eq!(joypad.read_addr(0x4016), 0x00);
  }
}
