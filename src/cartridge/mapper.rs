#[derive(PartialEq, Debug, Clone)]
pub enum MapperType {
    NROM, // No mapper
    NintendoMMC1,
    CNROMSwitch,
    INESMapper211, // https://wiki.nesdev.com/w/index.php/INES_Mapper_211
}

pub trait Mapper {
    fn map(&self, in_addr: u16) -> u16;
}

pub fn create_mapper(t: MapperType) -> Box<Mapper> {
    match t {
        MapperType::NROM => Box::new(NROM{}),
        MapperType::CNROMSwitch => Box::new(CNROMSwitch{offset: 0u16}),
        _ => panic!("Not implemented."),
    }
}

struct NROM {}

impl Mapper for NROM {
    fn map(&self, in_addr: u16) -> u16 {
        in_addr
    }
}

struct CNROMSwitch {
    // TODO(tobys): This implementation is bunk. Do a proper one.
    offset: u16,
}

impl Mapper for CNROMSwitch {
    // TODO(tobys): This implementation is bunk. Do a proper one.
    fn map(&self, in_addr: u16) -> u16 {
        in_addr + self.offset
    }
}
