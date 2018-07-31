use ppu::palette::Color;
use sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

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

pub struct SdlVideoOutput<'a> {
  canvas: sdl2::render::Canvas<sdl2::video::Window>,
  texture: sdl2::render::Texture<'a>,
  frame_data: Vec<Color>,
  col: usize,
  line: usize,
}

impl<'a> SdlVideoOutput<'a> {
  pub fn new(
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
  ) -> Self {
    let texture = texture_creator
      .create_texture_streaming(PixelFormatEnum::RGB24, 256, 240)
      .unwrap();

    SdlVideoOutput {
      col: 0,
      line: 0,
      texture: texture,
      canvas: canvas,
      frame_data: vec![Color(0, 0, 0); 256 * 240],
    }
  }
}

impl<'a> VideoOutput for SdlVideoOutput<'a> {
  fn output_pixel(&mut self, c: Color) {
    if self.col >= 256 || self.line >= 240 {
      // Overscan, ignore
      return;
    }
    self.frame_data[self.line * 256 + self.col] = c;

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

    let data = &(self.frame_data);

    self
      .texture
      .with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for x in 0..256 {
          for y in 0..240 {
            let c = &data[y * 256 + x];
            let offset: usize = (y * pitch) + x * 3;
            buffer[offset] = c.0;
            buffer[offset + 1] = c.1;
            buffer[offset + 2] = c.2;
          }
        }
      })
      .unwrap();

    self.canvas.clear();
    self
      .canvas
      .copy(&self.texture, None, Some(Rect::new(0, 0, 256, 240)))
      .unwrap();
    self.canvas.present();
  }
}
