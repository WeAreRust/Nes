use bytes::BytesMut;

pub struct Memory {
    bytes: BytesMut,
}

impl Memory {
    pub fn with_bytes(bytes: BytesMut) -> Self {
        Memory { bytes }
    }

    pub fn fetch(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }

    pub fn store(&mut self, addr: u16, value: u8) -> u8 {
        let old = self.fetch(addr);
        self.bytes[addr as usize] = value;
        old
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_data_at_addr() {
        let bytes = BytesMut::from(vec![123]);
        let memory = Memory { bytes };

        assert_eq!(memory.fetch(0), 123);
    }

    #[test]
    fn store_data_at_addr() {
        let bytes = BytesMut::from(vec![0; 2]);
        let mut memory = Memory { bytes };

        assert_eq!(memory.store(1, 123), 0);
        assert_eq!(memory.bytes[1], 123);
    }
}
