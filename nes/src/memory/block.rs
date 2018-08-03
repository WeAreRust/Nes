use memory::{ReadAddr, WriteAddr};

use bytes::BytesMut;

pub struct BlockMemory {
  bytes: BytesMut,
}

impl BlockMemory {
  pub fn with_size(size: usize) -> Self {
    BlockMemory::with_bytes(vec![0x00; size])
  }

  pub fn with_bytes<B: Into<BytesMut>>(bytes: B) -> Self {
    BlockMemory {
      bytes: bytes.into(),
    }
  }
}

impl ReadAddr for BlockMemory {
  fn read_addr(&mut self, addr: u16) -> u8 {
    self.bytes[usize::from(addr)]
  }
}

impl WriteAddr for BlockMemory {
  fn write_addr(&mut self, addr: u16, value: u8) -> u8 {
    let old = self.read_addr(addr);
    self.bytes[usize::from(addr)] = value;
    old
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_addr() {
    let bytes = BytesMut::from(vec![123]);
    let mut memory = BlockMemory { bytes };

    assert_eq!(memory.read_addr(0), 123);
  }

  #[test]
  fn write_addr() {
    let bytes = BytesMut::from(vec![0; 2]);
    let mut memory = BlockMemory { bytes };

    assert_eq!(memory.write_addr(1, 123), 0);
    assert_eq!(memory.bytes[1], 123);
  }
}
