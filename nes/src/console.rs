use cartridge::Cartridge;
use clock;
use clock::{Clock, Processor};
use io::video::VideoOutput;

use bus::Bus;
use controller::Controller;
use cpu;
use memory::block::BlockMemory;
use ppu;

pub struct Console<'a, C1: 'a + Controller, C2: 'a + Controller> {
  clock: Clock,
  cpu: cpu::Core,
  ppu: ppu::Core,
  bus: Bus<'a, C1, C2>,
  cpu_interval: u8,
  ppu_interval: u8,
}

impl<'a, C1: 'a + Controller, C2: 'a + Controller> Console<'a, C1, C2> {
  pub fn new(
    cartridge: &'a mut Cartridge,
    controller1: Option<&'a mut C1>,
    controller2: Option<&'a mut C2>,
    video_output: impl VideoOutput + 'static,
  ) -> Self {
    let ram: Box<BlockMemory> = Box::new(BlockMemory::with_size(0x0800));

    Self {
      clock: Clock::new(),
      cpu: cpu::Core::default(),
      ppu: ppu::Core::new(Box::new(video_output)),
      bus: Bus::new(cartridge, ram, controller1, controller2),
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

    if self.ppu_interval == clock::PPU_PERIOD {
      self.ppu_interval = 0;
      self.ppu.cycle(&mut self.bus);
    }
  }
}
