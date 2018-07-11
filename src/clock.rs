use memory::Memory;

/// NTSC master clock frequency (per second)
pub const MASTER_FREQUENCY: usize = 21_477_272;

/// NTSC CPU frequency factor
pub const CPU_FREQUENCY: usize = 12;

/// NTSC PPU frequency factor
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
