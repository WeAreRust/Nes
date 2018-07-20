pub mod block;

pub trait ReadAddr {
    fn read_addr(&self, addr: u16) -> u8;
}

pub trait WriteAddr {
    type Width = u16;
    type Value = u8;

    fn write_addr(&mut self, addr: u16, value: u8) -> u8;
}
