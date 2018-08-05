use ppu::palette::Color;
use std::sync::mpsc;

/// A video output simulates a composite video output,
/// outputting one pixel at a time as well as extra
/// pulses to mark the beginning of new scanlines
/// or frames.
pub trait VideoOutput {
  /// Add a pixel and move on to the next
  fn output_pixel(&mut self, Color);

  /// Mark the beginning of a new scanline
  fn horizontal_sync(&mut self);

  /// Mark the beginning of a new frame
  fn vertical_sync(&mut self);
}

pub struct VideoFrame {
  /// 2D Vec of pixel values where the first dimension is the line
  pub frame_data: Vec<Vec<Color>>,
}

impl VideoFrame {
  /// Write a video frame to a flat array of RGB values (eg,
  /// to raw texture data)
  pub fn write_to_buffer(&self, buf: &mut [u8], pitch: usize) {
    for (y, line) in self.frame_data.iter().enumerate() {
      for (x, color) in line.iter().enumerate() {
        let offset: usize = (y * pitch) + x * 3;
        buf[offset] = color.0;
        buf[offset + 1] = color.1;
        buf[offset + 2] = color.2;
      }
    }
  }
}

/// A VideoOutput that sends frames over a synchronous channel
pub struct ChannelVideoOutput {
  sender: mpsc::SyncSender<VideoFrame>,
  frame_data: Vec<Vec<Color>>,
  col: usize,
  line: usize,
}

impl ChannelVideoOutput {
  pub fn new() -> (Self, mpsc::Receiver<VideoFrame>) {
    let (send, recv) = mpsc::sync_channel(2);
    (
      ChannelVideoOutput {
        sender: send,
        col: 0,
        line: 0,
        frame_data: vec![vec![Color(0, 0, 0); 256]; 240],
      },
      recv,
    )
  }
}

impl VideoOutput for ChannelVideoOutput {
  fn output_pixel(&mut self, c: Color) {
    if self.col >= 256 || self.line >= 240 {
      // Overscan, ignore
      return;
    }
    self.frame_data[self.line][self.col] = c;

    self.col += 1;
  }

  fn horizontal_sync(&mut self) {
    self.line += 1;
    self.col = 0;
  }

  /// A vertical sync sends the previous frame's data to the GPU as
  /// a texture, then renders that texture to the SDL window
  fn vertical_sync(&mut self) {
    self.col = 0;
    self.line = 0;

    self
      .sender
      .send(VideoFrame {
        frame_data: self.frame_data.clone(),
      }).unwrap();
  }
}
