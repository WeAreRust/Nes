use cartridge::Cartridge;
use clock;
use clock::{Clock, Processor};

use bus::Bus;
use controller::Controller;
use cpu::Core;
use memory::block::BlockMemory;

pub struct Console<'a, C1: 'a + Controller> {
  clock: Clock,
  cpu: Core,
  bus: Bus<'a, C1>,
  cpu_interval: u8,
  ppu_interval: u8,
}

impl<'a, C1: 'a + Controller> Console<'a, C1> {
  pub fn new(cartridge: &'a mut Cartridge, controller1: &'a mut C1) -> Self {
    let ram: Box<BlockMemory> = Box::new(BlockMemory::with_size(0x0800));

    Self {
      clock: Clock::new(),
      cpu: Core::default(),
      bus: Bus::new(cartridge, ram, controller1),
      cpu_interval: 0,
      ppu_interval: 0,
    }
  }

  // Power on the console.
  pub fn reset(&mut self) {
    self.cpu.reset(&mut self.bus);
  }

  pub fn tick(&mut self) {
    self.clock.cycle();
    self.cpu_interval += 1;
    self.ppu_interval += 1;

    if self.cpu_interval == clock::CPU_PERIOD {
      self.cpu_interval = 0;
      self.cpu.cycle(&mut self.bus);
    }

    if self.ppu_interval == clock::PPU_FREQUENCY {
      self.ppu_interval = 0;
      // TODO: ppu.cycle()
    }
  }
}
