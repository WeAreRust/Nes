use cartridge::mapper::Mapper;
use memory::{ReadAddr};

pub struct NROM {
    rom_data: Vec<u8>
}

impl NROM {
    pub fn new(rom_data: Vec<u8>) -> NROM {
        NROM {
            rom_data: rom_data,
        }
    }
}

impl Mapper for NROM {
}

impl ReadAddr<u16, u8> for NROM {
    fn read_addr(&self, r_addr: u16) -> u8 {
        // Memory locations $0000-$07FF are mirrored three times at 
        // $0800-$0FFF, $1000-$17FF, and $1800-$1FFF.
        match r_addr {
            0x0000...0x07FF => self.rom_data[r_addr as usize],
            _ => self.read_addr(r_addr & 0x07FF),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nrom_read_addr() {
        let rom_data = vec![0x01, 0x4c, 0xb8, 0xe3, 0x94, 0x00, 0xed, 0xdf];
        let nrom = NROM::new(rom_data);

        let byte_read = nrom.read_addr(0x03);

        assert_eq!(byte_read, 0xe3);
    }
}