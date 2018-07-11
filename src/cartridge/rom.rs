use cartridge::mapper;
use cartridge::mapper::MapperType;

pub struct Rom {
    content: Vec<u8>,
    mapper: MapperType,
}

impl Rom {
    pub fn new(content: Vec<u8>, mapper: MapperType) -> Self {
        Rom {
            content: content,
            mapper: mapper,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        let mapper = mapper::create_mapper(self.mapper.clone());
        let mapped_addr = mapper.map(addr) as usize;

        *self.content.get(mapped_addr).unwrap()
    }
}
