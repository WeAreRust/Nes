//! NROM Mememory Mapper (No Mapper)
//! 
//! NROM is the simplist mapper to implement as it is just the natural
//! behaviour of the NES system. 
//! 
//! NROM supports either 1 or 2 banks of PRG-ROM and no CHR-ROM.

use cartridge::mapper::Mapper;
use memory::{ReadAddr};

pub struct NROM {
    prg_rom: Vec<u8>,
    num_prg_rom_banks: u8,
}

impl NROM {
    pub fn new(prg_rom: Vec<u8>, num_prg_rom_banks: u8) -> NROM {
        NROM {
            prg_rom: prg_rom,
            num_prg_rom_banks: num_prg_rom_banks,
        }
    }
}

impl Mapper for NROM {
}

impl ReadAddr for NROM {
    fn read_addr(&self, r_addr: u16) -> u8 {
        match r_addr {
            // $8000-$FFFF is PRG-ROM data.
            // $8000-$BFFF is the first bank of PRG-ROM data.
            0x8000...0xBFFF => self.prg_rom[(r_addr - 0x8000) as usize],
            // $C000-$FFFF is either the second bank of PRG-ROM data, 
            // if present, or else a mirror of $8000-$BFFF.
            0xC000...0xFFFF if self.num_prg_rom_banks == 0x01 => self.prg_rom[(r_addr - 0xC000) as usize],
            0xC000...0xFFFF => self.prg_rom[(r_addr - 0x8000) as usize],
            _ => panic!("Reading outside cartridge range.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nrom_read_addr_prg() {
        let prg_rom = vec![0x01, 0x4c, 0xb8, 0xe3, 0x94, 0x00, 0xed, 0xdf];
        let nrom = NROM::new(prg_rom, 1);

        let byte_read = nrom.read_addr(0x8003);

        assert_eq!(byte_read, 0xe3);
    }

    #[test]
    fn nrom_read_mirrored() {
        let prg_rom = vec![0x01, 0x4c, 0xb8, 0xe3, 0x94, 0x00, 0xed, 0xdf];
        let nrom = NROM::new(prg_rom, 1);

        let byte_read = nrom.read_addr(0xC004); // 0xC000 + 0x0004

        assert_eq!(byte_read, 0x94);
    }
}
