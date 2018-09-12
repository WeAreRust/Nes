use io::video::VideoOutput;
use ppu::palette::Color;
use ppu::vram::Memory;

pub struct Core {
  scanline: u16,
  cycle: u16,
  video_output: Box<VideoOutput>,
  vram: Memory,
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
    let vram = Memory::default();

    Core {
      // Start on pre-render scanline
      scanline: 261,
      cycle: 0,
      video_output,
      vram: vram,
    }
  }

  /// The prerender scanline loads
  fn cycle_vblank(&mut self) {
    if self.cycle == 1 {
      // Set vblank flag
    }
  }
  fn cycle_visible(&mut self, _render: bool) {
    self
      .video_output
      .output_pixel(Color(self.cycle as u8, (self.cycle >> 8) as u8, 0x30));
  }

  pub fn cycle(&mut self) {
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
      261 => self.cycle_visible(false),

      // Postrender - PPU just idles on this scanline
      240 => (),

      // Vertical blanking scanlines (240-260)
      s if s > 240 => self.cycle_vblank(),

      // Visible scanline (0-240)
      _ => self.cycle_visible(true),
    }
  }
}
