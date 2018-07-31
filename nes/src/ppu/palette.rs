//! # Colour Palette
//!
//! From: Loopy, “NES Palette”, NesDev, http://nesdev.parodius.com/pal.txt
//!
//! http://nesdev.com/NESDoc.pdf (page 45)
//!
//! Maps a colour palette entry to an RGB value.

#[derive(Clone, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

static PALETTE: &'static [Color] = &[
  // 0x00
  Color(0x75, 0x75, 0x75),
  Color(0x27, 0x1B, 0x8F),
  Color(0x00, 0x00, 0x00),
  Color(0x47, 0x00, 0x9F),
  Color(0x8F, 0x00, 0x77),
  Color(0xAB, 0x00, 0x13),
  Color(0xA7, 0x00, 0x00),
  Color(0x7F, 0x0B, 0x00),
  Color(0x43, 0x2F, 0x00),
  Color(0x00, 0x47, 0x00),
  // 0A
  Color(0x00, 0x51, 0x00),
  Color(0x00, 0x3F, 0x17),
  Color(0x1B, 0x3F, 0x5F),
  Color(0x00, 0x00, 0x00),
  Color(0x00, 0x00, 0x00),
  Color(0x00, 0x00, 0x00),
  // 10
  Color(0xBC, 0xBC, 0xBC),
  Color(0x00, 0x73, 0xEF),
  Color(0x23, 0x3B, 0xEF),
  Color(0x83, 0x00, 0xF3),
  Color(0xBF, 0x00, 0xBF),
  Color(0xE7, 0x00, 0x5B),
  Color(0xDB, 0x2B, 0x00),
  Color(0xCB, 0x4F, 0x0F),
  Color(0x8B, 0x73, 0x00),
  Color(0x00, 0x97, 0x00),
  // 1A
  Color(0x00, 0xAB, 0x00),
  // TODO: 1B - 3F
];

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn palette_contains_green() {
    assert!(PALETTE[0x1A] == Color(0x00, 0xAB, 0x00));
  }
}
