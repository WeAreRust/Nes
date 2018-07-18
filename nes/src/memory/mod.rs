use bytes::BytesMut;

pub struct Memory {
    bytes: BytesMut,
}

impl Memory {
    pub fn with_bytes<B: Into<BytesMut>>(bytes: B) -> Self {
        Memory {
            bytes: bytes.into(),
        }
    }
}

pub trait ReadAddr<A, T> {
    fn read_addr(&self, addr: A) -> T;
}

impl ReadAddr<u16, u8> for Memory {
    fn read_addr(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }
}

pub trait WriteAddr<A, T> {
    fn write_addr(&mut self, addr: A, value: T) -> T;
}

impl WriteAddr<u16, u8> for Memory {
    fn write_addr(&mut self, addr: u16, value: u8) -> u8 {
        let old = self.read_addr(addr);
        self.bytes[addr as usize] = value;
        old
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_addr() {
        let bytes = BytesMut::from(vec![123]);
        let memory = Memory { bytes };

        assert_eq!(memory.read_addr(0), 123);
    }

    #[test]
    fn write_addr() {
        let bytes = BytesMut::from(vec![0; 2]);
        let mut memory = Memory { bytes };

        assert_eq!(memory.write_addr(1, 123), 0);
        assert_eq!(memory.bytes[1], 123);
    }
}
