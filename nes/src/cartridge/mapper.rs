use cartridge::mappers::nrom::NROM;
use memory::{ReadAddr};

#[derive(PartialEq, Debug, Clone)]
pub enum MapperType {
    NROM, // No mapper
    NintendoMMC1,
    CNROMSwitch,
    INESMapper211, // https://wiki.nesdev.com/w/index.php/INES_Mapper_211
}

pub trait Mapper: ReadAddr<u16, u8> {
}

impl Mapper {
    pub fn create(
        t: MapperType,
        prg_rom_data: Vec<u8>,
        _chr_rom_data: Vec<u8>,
        num_prg_rom_banks: u8,
        _num_chr_rom_banks: u8,
    ) -> Box<Mapper> {
        match t {
            MapperType::NROM => Box::new(
                NROM::new(prg_rom_data, num_prg_rom_banks)
            ),
            _ => panic!("Mapper not implemented."),
        }
    }
}

