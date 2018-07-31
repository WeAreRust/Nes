use cartridge::Cartridge;
use clock;
use clock::{Clock, Processor};

use bus::Bus;
use cpu::Core;
use memory::block::BlockMemory;

pub struct Console<'a> {
  pub cartridge: &'a mut Cartridge,
}

impl<'a> Console<'a> {
  pub fn run(&mut self) {
    let mut clock = Clock::new();
    let mut cpu = Core::default();
    let mut ram = BlockMemory::with_size(0x0800);
    let mut bus = Bus::new(&mut self.cartridge, &mut ram);
    let mut cpu_interval: u8 = 0;
    let mut ppu_interval: u8 = 0;

    cpu.reset(&bus);
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
