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
    pub fn create(t: MapperType, rom_data: Vec<u8>) -> Box<Mapper> {
        match t {
            MapperType::NROM => Box::new(NROM::new(rom_data)),
            _ => panic!("Mapper not implemented."),
        }
    }
}

