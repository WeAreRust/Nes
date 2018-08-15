use cartridge::Cartridge;
use controller::Controller;
use memory::block::BlockMemory;
use memory::{ReadAddr, WriteAddr};

pub struct Bus<'a, C1: 'a + Controller, C2: 'a + Controller> {
  cartridge: &'a mut Cartridge,
  ram: Box<BlockMemory>,
  controller1: Option<&'a mut C1>,
  controller2: Option<&'a mut C2>,
}

impl<'a, C1: Controller, C2: Controller> Bus<'a, C1, C2> {
  pub fn new(
    cartridge: &'a mut Cartridge,
    ram: Box<BlockMemory>,
    controller1: Option<&'a mut C1>,
    controller2: Option<&'a mut C2>,
  ) -> Self {
    Bus {
      cartridge: cartridge,
      ram: ram,
      controller1: controller1,
      controller2: controller2,
    }
  }
}

impl<'a, C1: Controller, C2: Controller> ReadAddr for Bus<'a, C1, C2> {
  fn read_addr(&mut self, addr: u16) -> u8 {
    match addr {
      // RAM
      0x0000...0x1FFF => self.ram.read_addr(addr & 0x07FF),
      // I/O Registers
      0x2000...0x3FFF => {
        let _mirrored_addr = ((addr - 0x2000) & 0x0007) + 0x2000;
        // TODO: Send to PPU
        0x00
      }
      0x4000...0x4015 => {
        // TODO: Send to APU
        0x00
      }
      // Controller 1
      0x4016 => match &mut self.controller1 {
        Some(controller) => controller.read_addr(addr),
        None => 0x00,
      },
      // Controller 2
      0x4017 => match &mut self.controller2 {
        Some(controller) => controller.read_addr(addr),
        None => 0x00,
      },
      // APU and I/O functionality that is usually disabled
      0x4018...0x401F => panic!("Attempted access to disabled I/O ${:04X}", addr),
      // TODO: Expansion ROM
      0x4020...0x5FFF => panic!("Attempted access to expansion ROM ${:04X}", addr),
      // TODO: Save RAM
      0x6000...0x7FFF => panic!("Attempted access to Save RAM ${:04X}", addr),
      // Catridge ROM
      0x8000...0xFFFF => self.cartridge.mapper.read_addr(addr),
      _ => panic!("Bus addr not implemented for ${:04X}", addr),
    }
  }
}

impl<'a, C1: Controller, C2: Controller> WriteAddr for Bus<'a, C1, C2> {
  fn write_addr(&mut self, addr: u16, value: u8) -> u8 {
    match addr {
      // RAM
      0x0000...0x1FFF => self.ram.write_addr(addr & 0x07FF, value),
      // I/O Registers
      0x2000...0x3FFF => {
        let _mirrored_addr = ((addr - 0x2000) & 0x0007) + 0x2000;
        // TODO: Send to PPU
        0x00
      }
      0x4000...0x4015 => {
        // TODO: Send to APU
        0x00
      }
      // Controller 1
      0x4016 => match &mut self.controller1 {
        Some(controller) => controller.write_addr(addr, value),
        None => 0x00,
      },
      // Controller 2
      0x4017 => match &mut self.controller2 {
        Some(controller) => controller.write_addr(addr, value),
        None => 0x00,
      },
      // APU and I/O functionality that is usually disabled
      0x4018...0x401F => panic!("Attempted access to disabled I/O ${:04X}", addr),
      // TODO: Expansion ROM
      0x4020...0x5FFF => panic!("Attempted access to expansion ROM ${:04X}", addr),
      // TODO: Save RAM
      0x6000...0x7FFF => panic!("Attempted access to Save RAM ${:04X}", addr),
      // Catridge ROM
      0x8000...0xFFFF => self.cartridge.mapper.write_addr(addr, value),
      _ => panic!("Bus addr not implemented for ${:04X}", addr),
    }
  }
}
