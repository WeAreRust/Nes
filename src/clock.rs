use memory::Memory;

/// NTSC master clock frequency (per second)
pub const MASTER_FREQUENCY: usize = 21_477_272;

/// NTSC CPU divisor
///
/// Used to calculate the frequency of the CPU = `MASTER_FREQUENCY / CPU_PERIOD`.
pub const CPU_PERIOD: usize = 12;

/// NTSC PPU divisor
///
/// Used to calculate the frequency of the PPU = `MASTER_FREQUENCY / PPU_PERIOD`.
pub const PPU_FREQUENCY: usize = 4;

pub trait Processor {
    fn cycle(&mut self, memory: &mut Memory);
}

pub struct Clock {
    tick: usize,
}

impl Clock {
    pub fn cycle() {
        unimplemented!();
    }
}
