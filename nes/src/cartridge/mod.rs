//! A NES cartridge includes the the game data, stored in ROM.
//! The cartridge may also include a memory mapper (MMC) and/or
//! battery-powered RAM for game save data.
//!
//! A memory mapper will allow a cartridge to swap memory out and utilize memory
//! address space beyond $FFFF.
//!
//! NROM indicates no mapper is present.

mod ines;
mod mapper;
mod mappers;
mod mirroring;

use cartridge::mapper::Mapper;
use cartridge::mirroring::Mirroring;

pub struct Cartridge {
  pub mirroring: Mirroring,
  pub battery_ram_present: bool,
  pub mapper: Box<Mapper>,
}

impl Cartridge {
  fn try_from_ines(image: ines::Image) -> Result<Self, ParseError> {
    println!("iNES Image: {:?}", image);

    Ok(Cartridge {
      mirroring: image.mirror,
      battery_ram_present: image.has_battery_ram,
      mapper: Mapper::create(
        image.mapper,
        image.prg_rom_data,
        image.chr_rom_data,
        image.num_prg_banks,
        image.num_chr_banks,
      ),
    })
  }
}

#[derive(PartialEq, Debug)]
struct UnknownFormat {}

#[derive(PartialEq, Debug)]
pub enum ParseError {
  UnknownFormat,
  InvalidFile,
}

impl From<UnknownFormat> for ParseError {
  fn from(_e: UnknownFormat) -> Self {
    ParseError::UnknownFormat
  }
}

impl From<ines::ParseError> for ParseError {
  fn from(_e: ines::ParseError) -> Self {
    ParseError::InvalidFile
  }
}

#[derive(PartialEq, Debug)]
enum Format {
  INES,
}

pub fn parse_rom_file(data: &[u8]) -> Result<Cartridge, ParseError> {
  match detect_format(data)? {
    Format::INES => parse_ines(data),
  }
}

fn detect_format(data: &[u8]) -> Result<Format, UnknownFormat> {
  if ines::check_format(data) {
    Ok(Format::INES)
  } else {
    Err(UnknownFormat {})
  }
}

fn parse_ines(data: &[u8]) -> Result<Cartridge, ParseError> {
  let rom: ines::Image = ines::parse_ines(data)?;

  Cartridge::try_from_ines(rom)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_detect_format_ines() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
    assert_eq!(detect_format(&data), Ok(Format::INES));
  }

  #[test]
  pub fn test_parse_rom_ines() {
    let mut data = [00u8; 49168];
    data[..8].clone_from_slice(&[0x4e, 0x45, 0x53, 0x1a, 0x02, 0x02, 0x01, 0x00]);

    let cartridge = parse_rom_file(&data).unwrap();
    assert_eq!(cartridge.mirroring, Mirroring::Vertical);
  }
}
