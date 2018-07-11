use cartridge::mapper::Mapper;
use cartridge::mirroring::Mirroring;

const INES_HEADER: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a];

const LEN_NES: usize = 4;
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

pub struct Rom {
    pub mirror: Mirroring,
    pub mapper: Mapper,
    pub four_screen_mirroring: bool,
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

pub fn parse_rom(data: &[u8]) -> Result<Rom, ParseError> {
    Ok(Rom {
        mirror: detect_mirror_type(data),
        mapper: detect_mapper(data)?,
        four_screen_mirroring: has_four_screen_mirroring(data),
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

fn detect_mapper(data: &[u8]) -> Result<Mapper, ParseErrorReason> {
    let mapper_num = (data[IDX_CB1] & CB1_MASK_MAPPER) >> 4 | (data[IDX_CB2] & CB2_MASK_MAPPER);
    // Find all known mapper numbers at https://wiki.nesdev.com/w/index.php/Mapper
    match mapper_num {
        MAPPER_NROM => Ok(Mapper::NROM),
        MAPPER_NINTENDO_MMC1 => Ok(Mapper::NintendoMMC1),
        MAPPER_CNROM_SWITCH => Ok(Mapper::CNROMSwitch),
        MAPPER_INES_211 => Ok(Mapper::INESMapper211),
        _ => Err(ParseErrorReason::UnknownMapper),
    }
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
        assert_eq!(detect_mapper(&data), Ok(Mapper::NROM));
    }

    #[test]
    pub fn test_detect_mapper_mmc1() {
        let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x10, 0x00];
        assert_eq!(detect_mapper(&data), Ok(Mapper::NintendoMMC1));
    }

    #[test]
    pub fn test_detect_mapper_cnrom_switch() {
        let data = [0x4e, 0x45, 0x53, 0x1a, 0x02, 0x02, 0x31, 0x00];
        assert_eq!(detect_mapper(&data), Ok(Mapper::CNROMSwitch));
    }

    #[test]
    pub fn test_detect_mapper_ines211() {
        let data = [0x4e, 0x45, 0x53, 0x1a, 0x10, 0x20, 0x30, 0xd0];
        assert_eq!(detect_mapper(&data), Ok(Mapper::INESMapper211));
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

        let rom = parse_rom(&data).unwrap();
        assert_eq!(rom.mirror, Mirroring::Vertical);
        assert_eq!(rom.mapper, Mapper::CNROMSwitch);
        assert_eq!(rom.four_screen_mirroring, false);
    }
}
