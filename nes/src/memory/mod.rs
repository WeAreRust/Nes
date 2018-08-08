pub mod block;

pub trait ReadAddr {
  fn read_addr(&mut self, addr: u16) -> u8;
}

pub trait WriteAddr: ReadAddr {
  fn write_addr(&mut self, addr: u16, value: u8) -> u8;
}
