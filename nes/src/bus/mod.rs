use apu::Apu;
use cartridge::Cartridge;
use controller::Controller;
use io::video::VideoOutput;
use memory::block::BlockMemory;
use memory::{ReadAddr, WriteAddr};
use ppu;

pub struct Bus<'a, C1: 'a + Controller, C2: 'a + Controller, A1: 'a + Apu> {
  cartridge: &'a mut Cartridge,
  ram: Box<BlockMemory>,
  apu: &'a mut A1,
  pub ppu: ppu::Core,
  controller1: Option<&'a mut C1>,
  controller2: Option<&'a mut C2>,
}

impl<'a, C1: Controller, C2: Controller, A1: Apu> Bus<'a, C1, C2, A1> {
  pub fn new(
    cartridge: &'a mut Cartridge,
    ram: Box<BlockMemory>,
    apu: &'a mut A1,
    controller1: Option<&'a mut C1>,
    controller2: Option<&'a mut C2>,
    video_output: impl VideoOutput + 'static,
  ) -> Self {
    Bus {
      cartridge: cartridge,
      ram: ram,
      apu: apu,
      ppu: ppu::Core::new(Box::new(video_output)),
      controller1: controller1,
      controller2: controller2,
    }
  }
}

impl<'a, C1: Controller, C2: Controller, A1: Apu> ReadAddr for Bus<'a, C1, C2, A1> {
  fn read_addr(&mut self, addr: u16) -> u8 {
    match addr {
      // RAM
      0x0000...0x1FFF => self.ram.read_addr(addr & 0x07FF),
      // I/O Registers
      0x2000...0x3FFF => {
        let mirrored_addr = addr & 0x2007;
        self.ppu.read_addr(mirrored_addr)
      }
      0x4000...0x4013 => self.apu.read_addr(addr),
      0x4014 => panic!("Attempted illegal read from {:04X}", addr),
      0x4015 => self.apu.read_addr(addr),
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

impl<'a, C1: Controller, C2: Controller, A1: Apu> WriteAddr for Bus<'a, C1, C2, A1> {
  fn write_addr(&mut self, addr: u16, value: u8) -> u8 {
    match addr {
      // RAM
      0x0000...0x1FFF => self.ram.write_addr(addr & 0x07FF, value),
      // I/O Registers
      0x2000...0x3FFF => {
        let mirrored_addr = addr & 0x2007;
        self.ppu.write_addr(mirrored_addr, value)
      }
      0x4000...0x4013 => self.apu.write_addr(addr, value),
      0x4014 => {
        // TODO: DMA to ppu
        unimplemented!("DMA not implemented");
      }
      0x4015 => self.apu.write_addr(addr, value),
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
