use cartridge::Cartridge;
use clock;
use clock::{Clock, Processor};

use bus::Bus;
use controller::Controller;
use cpu::Core;
use memory::block::BlockMemory;

pub struct Console<'a, C1: 'a + Controller> {
  pub cartridge: &'a mut Cartridge,
  pub controller1: &'a mut C1,
}

impl<'a, C1: 'a + Controller> Console<'a, C1> {
  pub fn run(&mut self) {
    let mut clock = Clock::new();
    let mut cpu = Core::default();
    let mut ram = BlockMemory::with_size(0x0800);
    let mut bus = Bus::new(&mut self.cartridge, &mut ram, self.controller1);
    let mut cpu_interval: u8 = 0;
    let mut ppu_interval: u8 = 0;

    cpu.reset(&mut bus);
    loop {
      clock.cycle();
      cpu_interval += 1;
      ppu_interval += 1;

      if cpu_interval == clock::CPU_PERIOD {
        cpu_interval = 0;
        cpu.cycle(&mut bus);
      }

      if ppu_interval == clock::PPU_FREQUENCY {
        ppu_interval = 0;
        // TODO: ppu.cycle()
      }
    }
  }
}
