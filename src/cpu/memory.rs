const MEMORY_SIZE: usize = 65536;

pub type MemorySpace = [u8; MEMORY_SIZE];

pub struct Memory {
    data: MemorySpace,
}

impl Memory {
    pub fn with_data(data: MemorySpace) -> Self {
        // Remove this comment later, but having this here means that it won't compile if data
        // isn't of the correct memory size, since the size of [u8] is known at compile time.
        assert_eq!(MEMORY_SIZE, data.len());

        Memory { data }
    }

    pub fn fetch(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn store(&mut self, addr: u16, new: u8) -> u8 {
        let old = self.fetch(addr);
        self.data[addr as usize] = new;
        old
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            data: [0; MEMORY_SIZE],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_correct_data_size() {
        let memory = Memory::default();

        assert_eq!(memory.data.len(), MEMORY_SIZE);
    }

    #[test]
    fn fetch_data_at_addr() {
        let mut memory = Memory::default();
        memory.data[99] = 123;

        assert_eq!(memory.fetch(99), 123);
    }

    #[test]
    fn store_data_at_addr() {
        let mut memory = Memory::default();

        assert_eq!(memory.store(99, 123), 0);
        assert_eq!(memory.data[99], 123);
    }
}
