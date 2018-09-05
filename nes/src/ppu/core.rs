use clock::Processor;
use io::video::VideoOutput;
use memory::{ReadAddr, WriteAddr};
use ppu::palette::Color;

pub struct Core {
  scanline: u16,
  cycle: u16,
  video_output: Box<VideoOutput>,
}

struct DummyVideoOutput {}
impl VideoOutput for DummyVideoOutput {
  fn output_pixel(&mut self, _: Color) {}
  fn horizontal_sync(&mut self) {}
  fn vertical_sync(&mut self) {}
}

impl Default for Core {
  fn default() -> Self {
    let video_output = Box::from(DummyVideoOutput {});
    Core::new(video_output)
  }
}

impl Core {
  pub fn new(video_output: Box<VideoOutput>) -> Self {
    Core {
      // Start on pre-render scanline
      scanline: 261,
      cycle: 0,
      video_output,
    }
  }

  /// The prerender scanline loads
  fn cycle_vblank(&mut self, _memory: &mut ReadAddr) {
    if self.cycle == 1 {
      // Set vblank flag
    }
  }
  fn cycle_visible(&mut self, _memory: &mut ReadAddr, _render: bool) {
    self
      .video_output
      .output_pixel(Color(self.cycle as u8, (self.cycle >> 8) as u8, 0x30));
  }
}

impl<T: ReadAddr + WriteAddr> Processor<T> for Core {
  fn cycle(&mut self, memory: &mut T) {
    self.cycle += 1;
    if self.cycle == 342 {
      self.cycle = 0;
      self.scanline += 1;
      self.video_output.horizontal_sync();

      if self.scanline == 262 {
        self.scanline = 0;
        self.video_output.vertical_sync();
      }
    }

    match self.scanline {
      // Prerender - same as a visible scanline but nothing is drawn
      261 => self.cycle_visible(memory, false),

      // Postrender - PPU just idles on this scanline
      240 => (),

      // Vertical blanking scanlines (240-260)
      s if s > 240 => self.cycle_vblank(memory),

      // Visible scanline (0-240)
      _ => self.cycle_visible(memory, true),
    }
  }
}
