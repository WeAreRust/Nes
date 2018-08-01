//! iNES file format
//!
//! iNES is a binary format starting with a 16 byte header block.
//!
//! Starting | Length | Data
//! Byte     |        |
//! ---------|--------|--------------------------------------------
//! 0        | 3      | Constant value: 'NES' (0x4E 0x45 0x1A)
//! ---------|--------|--------------------------------------------
//! 3        | 1      | Constant value: 0x1A
//! ---------|--------|--------------------------------------------
//! 4        | 1      | Number of 16KB PRG-ROM banks
//! ---------|--------|--------------------------------------------
//! 5        | 1      | Number of 8KB CHR-ROM banks
//! ---------|--------|--------------------------------------------
//! 6        | 1      | Control byte 1
//!          |        | 76543210
//!          |        | ||||||||
//!          |        | |||||||+- Mirroring: 0: horizontal
//!          |        | |||||||              1: vertical
//!          |        | ||||||+-- 1: Battery-backed RAM present
//!          |        | |||||+--- 1: 512-byte trainer present
//!          |        | ||||+---- 1: Four-screen mirroring present
//!          |        | ++++----- Lower bits of mapper number
//! ---------|--------|--------------------------------------------
//! 7        | 1      | Control byte 2
//!          |        | 76543210
//!          |        | ||||||||
//!          |        | ||||++++- Reserved for future use,
//!          |        | ||||      should all be 0
//!          |        | ++++----- Upper bits of mapper number
//! ---------|--------|--------------------------------------------
//! 8        | 1      | Number of 8KB RAM banks. If 0, assume 1 for
//!          |        | backwards compatibility.
//! ---------|--------|--------------------------------------------
//! 9        | 7      | Reserved for future use. Should be 0.
//! ---------|--------|--------------------------------------------
//!
//! The 512-byte trainer immediately follows the header if it is
//! indicated as present (see header byte 6).
//!
//! Next, the PRG-ROM follows. The size is equal to #Banks * 16KB.
//!
//! And finally, the CHR-ROM. The size is equal to #Banks * 8KB.

use std::fmt;

use cartridge::mapper::MapperType;
use cartridge::mirroring::Mirroring;

const INES_HEADER: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a];

const LEN_NES: usize = 4;
const LEN_HEADER: usize = 16;
const LEN_TRAINER: usize = 512;

const IDX_NUM_PRG_ROM: usize = 4;
const IDX_NUM_CHR_ROM: usize = 5;
const IDX_CB1: usize = 6;
const IDX_CB2: usize = 7;

const CB1_BIT_MIRRORING: u8 = 0x01;
const CB1_BIT_BATTERY_RAM: u8 = 0x02;
const CB1_BIT_TRAINER: u8 = 0x04;
const CB1_BIT_FOUR_SCREEN_MIRRORING: u8 = 0x08;
const CB1_MASK_MAPPER: u8 = 0xF0;
const CB2_MASK_MAPPER: u8 = 0xF0;

const MAPPER_NROM: u8 = 0;
const MAPPER_NINTENDO_MMC1: u8 = 1;
const MAPPER_CNROM_SWITCH: u8 = 3;
const MAPPER_INES_211: u8 = 211;

const SIZE_PRG_ROM_BANK: usize = 16 * 1024;
const SIZE_CHR_ROM_BANK: usize = 8 * 1024;

pub struct Image {
  pub mirror: Mirroring,
  pub mapper: MapperType,
  pub four_screen_mirroring: bool,
  pub num_prg_banks: u8,
  pub prg_rom_data: Vec<u8>,
  pub num_chr_banks: u8,
  pub chr_rom_data: Vec<u8>,
  pub has_battery_ram: bool,
  has_trainer: bool,
}

impl fmt::Debug for Image {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Image{{ has_trainer: {} }}", self.has_trainer)
  }
}

#[derive(PartialEq, Debug)]
pub enum ParseErrorReason {
  UnknownMapper,
}

#[derive(PartialEq, Debug)]
pub struct ParseError {
  reason: ParseErrorReason,
}

impl From<ParseErrorReason> for ParseError {
  fn from(e: ParseErrorReason) -> Self {
    ParseError { reason: e }
  }
}

pub fn check_format(data: &[u8]) -> bool {
  data[..LEN_NES] == INES_HEADER
}

pub fn parse_ines(data: &[u8]) -> Result<Image, ParseError> {
  Ok(Image {
    mirror: detect_mirror_type(data),
    mapper: detect_mapper(data)?,
    four_screen_mirroring: has_four_screen_mirroring(data),
    num_prg_banks: count_prg_rom_banks(data),
    prg_rom_data: extract_prg_rom_data(data),
    num_chr_banks: count_chr_rom_banks(data),
    chr_rom_data: extract_chr_rom_data(data),
    has_trainer: has_trainer(data),
    has_battery_ram: has_battery_backed_ram(data),
  })
}

fn count_prg_rom_banks(data: &[u8]) -> u8 {
  data[IDX_NUM_PRG_ROM]
}

fn count_chr_rom_banks(data: &[u8]) -> u8 {
  data[IDX_NUM_CHR_ROM]
}

fn detect_mirror_type(data: &[u8]) -> Mirroring {
  match data[IDX_CB1] & CB1_BIT_MIRRORING == 0 {
    true => Mirroring::Horizontal,
    false => Mirroring::Vertical,
  }
}

fn has_battery_backed_ram(data: &[u8]) -> bool {
  data[IDX_CB1] & CB1_BIT_BATTERY_RAM != 0
}

fn has_trainer(data: &[u8]) -> bool {
  data[IDX_CB1] & CB1_BIT_TRAINER != 0
}

fn has_four_screen_mirroring(data: &[u8]) -> bool {
  data[IDX_CB1] & CB1_BIT_FOUR_SCREEN_MIRRORING != 0
}

fn detect_mapper(data: &[u8]) -> Result<MapperType, ParseErrorReason> {
  let mapper_num = (data[IDX_CB1] & CB1_MASK_MAPPER) >> 4 | (data[IDX_CB2] & CB2_MASK_MAPPER);

  // Find all known mapper numbers at https://wiki.nesdev.com/w/index.php/Mapper
  match mapper_num {
    MAPPER_NROM => Ok(MapperType::NROM),
    MAPPER_NINTENDO_MMC1 => Ok(MapperType::NintendoMMC1),
    MAPPER_CNROM_SWITCH => Ok(MapperType::CNROMSwitch),
    MAPPER_INES_211 => Ok(MapperType::INESMapper211),
    _ => Err(ParseErrorReason::UnknownMapper),
  }
}

fn prg_rom_start(data: &[u8]) -> usize {
  if has_trainer(data) {
    LEN_HEADER + LEN_TRAINER
  } else {
    LEN_HEADER
  }
}

fn extract_prg_rom_data(data: &[u8]) -> Vec<u8> {
  let prg_start = prg_rom_start(data);
  let len_prg = usize::from(count_prg_rom_banks(data)) * SIZE_PRG_ROM_BANK;
  data[prg_start..prg_start + len_prg].to_vec()
}

fn chr_rom_start(data: &[u8]) -> usize {
  let prg_start = prg_rom_start(data);
  let len_prg = usize::from(count_prg_rom_banks(data)) * SIZE_PRG_ROM_BANK;
  prg_start + len_prg
}

fn extract_chr_rom_data(data: &[u8]) -> Vec<u8> {
  let chr_start = chr_rom_start(data);
  let len_chr = usize::from(count_chr_rom_banks(data)) * SIZE_CHR_ROM_BANK;
  data[chr_start..chr_start + len_chr].to_vec()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_check_format() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
    assert!(check_format(&data));
  }

  #[test]
  pub fn test_count_prg_rom_banks() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
    assert_eq!(count_prg_rom_banks(&data), 16u8);
  }

  #[test]
  pub fn test_count_chr_rom_banks() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
    assert_eq!(count_chr_rom_banks(&data), 32u8);
  }

  #[test]
  pub fn test_detect_mirror_type_horizontal() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
    assert_eq!(detect_mirror_type(&data), Mirroring::Horizontal);
  }

  #[test]
  pub fn test_detect_mirror_type_vertical() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x31, 0xd0];
    assert_eq!(detect_mirror_type(&data), Mirroring::Vertical);
  }

  #[test]
  pub fn test_has_no_battery_backed_ram() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
    assert_eq!(has_battery_backed_ram(&data), false);
  }

  #[test]
  pub fn test_has_battery_backed_ram() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x32, 0xd0];
    assert_eq!(has_battery_backed_ram(&data), true);
  }

  #[test]
  pub fn test_has_no_trainer() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
    assert_eq!(has_trainer(&data), false);
  }

  #[test]
  pub fn test_has_trainer() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x34, 0xd0];
    assert_eq!(has_trainer(&data), true);
  }

  #[test]
  pub fn test_has_no_four_screen_mirroring() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
    assert_eq!(has_four_screen_mirroring(&data), false);
  }

  #[test]
  pub fn test_has_four_screen_mirroring() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x38, 0xd0];
    assert_eq!(has_four_screen_mirroring(&data), true);
  }

  #[test]
  pub fn test_detect_mapper_none() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x00, 0x00];
    assert_eq!(detect_mapper(&data), Ok(MapperType::NROM));
  }

  #[test]
  pub fn test_detect_mapper_mmc1() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x10, 0x00];
    assert_eq!(detect_mapper(&data), Ok(MapperType::NintendoMMC1));
  }

  #[test]
  pub fn test_detect_mapper_cnrom_switch() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x02, 0x02, 0x31, 0x00];
    assert_eq!(detect_mapper(&data), Ok(MapperType::CNROMSwitch));
  }

  #[test]
  pub fn test_detect_mapper_ines211() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
    assert_eq!(detect_mapper(&data), Ok(MapperType::INESMapper211));
  }

  #[test]
  pub fn test_detect_mapper_unknown() {
    let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0xA0, 0x50];
    assert_eq!(detect_mapper(&data), Err(ParseErrorReason::UnknownMapper));
  }

  #[test]
  pub fn test_parse_rom() {
    let mut data = [00u8; 49168];
    data[..8].clone_from_slice(&[0x4e, 0x45, 0x53, 0x1a, 0x02, 0x02, 0x31, 0x00]);

    let rom = parse_ines(&data).unwrap();
    assert_eq!(rom.mirror, Mirroring::Vertical);
    assert_eq!(rom.mapper, MapperType::CNROMSwitch);
    assert_eq!(rom.four_screen_mirroring, false);
  }
}
