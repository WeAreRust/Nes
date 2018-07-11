#[derive(PartialEq, Debug, Clone)]
pub enum MapperType {
    NROM, // No mapper
}

// TODO(toby): This mapper implementation is upside-down. Most mappers needs to support state.
pub trait Mapper {
    fn map(&self, in_addr: u16) -> u16;
}

pub fn create_mapper(t: MapperType) -> Box<Mapper> {
    match t {
        MapperType::NROM => Box::new(NROM{}),
        _ => panic!("Not implemented."),
    }
}

struct NROM {}

impl Mapper for NROM {
    fn map(&self, in_addr: u16) -> u16 {
        in_addr
    }
}
