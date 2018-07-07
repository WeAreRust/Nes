use self::basic::BasicMemory;

mod basic;

const NES_MEMORY_SIZE: usize = 65536;

pub trait Memory {
    fn fetch(&self, addr: u16) -> u8;
    fn store(&mut self, addr: u16, value: u8) -> u8;
}

pub type NesMemorySpace = [u8; NES_MEMORY_SIZE];

pub struct NesMemory {
    memory: BasicMemory<NesMemorySpace>,
}

impl NesMemory {
    pub fn with_data(data: NesMemorySpace) -> Self {
        NesMemory {
            memory: BasicMemory::with_data(data),
        }
    }

    fn fetch(&self, addr: u16) -> u8 {
        self.memory.fetch(addr)
    }

    fn store(&mut self, addr: u16, value: u8) -> u8 {
        self.memory.store(addr, value)
    }
}

impl Default for NesMemory {
    fn default() -> Self {
        NesMemory::with_data([0; NES_MEMORY_SIZE])
    }
}
