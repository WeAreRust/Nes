//! # PPU Memory (Video RAM)
//!
//! The PPU (picture processing unit) in the NES
//! has its own memory used for storing sprite data,
//! colour palettes etc.
//!
//! Although there is 64K of addressable memory, there
//! is only 16KB of physical memory so addresses beyond
//! 16KB are wrapped around (mirrored).
//!
//! Additionally, the name table and palette memory ranges
//! are wrapped towards the end of their range.

type Addr = u16;

pub struct Memory {
  bytes: Vec<u8>,
}

impl Default for Memory {
  fn default() -> Self {
    Memory {
      bytes: vec![0; 0x4000],
    }
  }
}

impl Memory {
  pub fn new() -> Self {
    Memory::default()
  }

  pub fn read_addr(&self, addr: Addr) -> u8 {
    let addr: usize = wrapped_addr(addr).into();
    self.bytes[addr]
  }

  pub fn write_addr(&mut self, addr: Addr, val: u8) {
    let addr: usize = wrapped_addr(addr).into();
    self.bytes[addr] = val;
  }
}

fn wrapped_addr(addr: Addr) -> Addr {
  match addr {
    // Name/attribute table mirroring
    0x3000...0x3EFF => 0x2000 + ((addr - 0x3000) % 0x0EFF),

    // Palette mirroring
    0x3F20...0x3FFF => 0x3F00 + ((addr - 0x3F20) % 0x0020),

    // Physical memory addressing
    0x0000...0x2FFF => addr,
    0x3F00...0x3F1F => addr,

    // Remaining address space mirroring
    0x4000...0xFFFF => wrapped_addr(addr % 0x4000),
    _ => panic!("Read PPU memory address out of bounds: {:x}", addr),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn wrapped_addr_wraps_name_tables() {
    assert_eq!(wrapped_addr(0x2012), 0x2012);
    assert_eq!(wrapped_addr(0x3012), 0x2012);
    assert_eq!(wrapped_addr(0x6012), 0x2012);
  }

  #[test]
  fn wrapped_addr_wraps_palettes() {
    assert_eq!(wrapped_addr(0x3F22), 0x3F02);
    assert_eq!(wrapped_addr(0x3F82), 0x3F02);
    assert_eq!(wrapped_addr(0x7F82), 0x3F02);
  }

  #[test]
  fn wrapped_addr_wraps_non_physical_address_space() {
    assert_eq!(wrapped_addr(0x4000), 0x0000);
    assert_eq!(wrapped_addr(0x5000), 0x1000);
    assert_eq!(wrapped_addr(0x6000), 0x2000);
    // Special case: 0x7000 maps to 0x3000, which is then
    // wrapped to 0x2000 because of the name table wrapping
    assert_eq!(wrapped_addr(0x7000), 0x2000);
    assert_eq!(wrapped_addr(0x8000), 0x0000);
  }

  #[test]
  fn read_addr() {
    let mut mem = Memory {
      bytes: vec![0; 0x4000],
    };
    mem.bytes[0x2000] = 15;
    assert_eq!(mem.read_addr(0x1FFF), 0);
    assert_eq!(mem.read_addr(0x2000), 15);
    assert_eq!(mem.read_addr(0x2001), 0);
  }

  #[test]
  fn read_addr_reads_wrapped_address() {
    let mut mem = Memory {
      bytes: vec![0; 0x4000],
    };
    mem.bytes[0x20FF] = 15;
    assert_eq!(mem.read_addr(0x20FF), 15);
    assert_eq!(mem.read_addr(0x30FF), 15);
    assert_eq!(mem.read_addr(0x60FF), 15);
    assert_eq!(mem.read_addr(0x70FF), 15);
  }

  #[test]
  fn write_addr() {
    let mut mem = Memory {
      bytes: vec![0; 0x4000],
    };
    mem.write_addr(0x1FFF, 254);
    mem.write_addr(0x2000, 255);
    mem.write_addr(0x2001, 1);

    assert_eq!(mem.bytes[0x1FFF], 254);
    assert_eq!(mem.bytes[0x2000], 255);
    assert_eq!(mem.bytes[0x2001], 1);
  }

  #[test]
  fn write_addr_writes_wrapped_address() {
    let mut mem = Memory {
      bytes: vec![0; 0x4000],
    };
    mem.write_addr(0x5FFF, 254);
    mem.write_addr(0x6000, 255);
    mem.write_addr(0x6001, 1);

    assert_eq!(mem.bytes[0x1FFF], 254);
    assert_eq!(mem.bytes[0x2000], 255);
    assert_eq!(mem.bytes[0x2001], 1);
  }
}
