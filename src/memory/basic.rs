use core::array::FixedSizeArray;

pub struct BasicMemory<T: FixedSizeArray<u8>> {
    data: T,
}

impl<T: FixedSizeArray<u8>> BasicMemory<T> {
    pub fn with_data(data: T) -> Self {
        BasicMemory { data }
    }

    pub fn fetch(&self, addr: u16) -> u8 {
        self.data.as_slice()[addr as usize]
    }

    pub fn store(&mut self, addr: u16, value: u8) -> u8 {
        let old = self.fetch(addr);
        self.data.as_mut_slice()[addr as usize] = value;
        old
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_data_at_addr() {
        let data = [0, 123];
        let memory = BasicMemory { data };

        assert_eq!(memory.fetch(1), 123);
    }

    #[test]
    fn store_data_at_addr() {
        let data = [0; 2];
        let mut memory = BasicMemory { data };

        assert_eq!(memory.store(1, 123), 0);
        assert_eq!(memory.data[1], 123);
    }
}
