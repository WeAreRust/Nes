use cartridge::Cartridge;
use controller::Controller;
use memory::block::BlockMemory;
use memory::{ReadAddr, WriteAddr};

pub struct Bus<'a, C1: 'a + Controller> {
  cartridge: &'a mut Cartridge,
  ram: Box<BlockMemory>,
  controller1: &'a mut C1,
}

impl<'a, C1: Controller> Bus<'a, C1> {
  pub fn new(cartridge: &'a mut Cartridge, ram: Box<BlockMemory>, controller1: &'a mut C1) -> Self {
    Bus {
      cartridge: cartridge,
      ram: ram,
      controller1: controller1,
    }
  }
}

impl<'a, C1: Controller> ReadAddr for Bus<'a, C1> {
  fn read_addr(&mut self, addr: u16) -> u8 {
    match addr {
      // RAM
      0x0000...0x1FFF => self.ram.read_addr(addr & 0x07FF),
      // Controller 1
      0x4016 => self.controller1.read_addr(addr),
      // Controller 2
      0x4017 => panic!("Controller 2 not implemented"),
      // Catridge ROM
      0x8000...0xFFFF => self.cartridge.mapper.read_addr(addr),
      _ => panic!("Bus addr not implemented for ${:04X}", addr),
    }
  }
}

impl<'a, C1: Controller> WriteAddr for Bus<'a, C1> {
  fn write_addr(&mut self, addr: u16, value: u8) -> u8 {
    match addr {
      // RAM
      0x0000...0x1FFF => self.ram.write_addr(addr & 0x07FF, value),
      // Catridge ROM
      0x8000...0xFFFF => self.cartridge.mapper.write_addr(addr, value),
      _ => panic!("Bus addr not implemented for ${:04X}", addr),
    }
  }
}
