use std::time::{Instant, Duration};
use std::thread;
use std::ops::Add;

use memory::{ReadAddr, WriteAddr};

/// NTSC master clock frequency (per second)
pub const MASTER_FREQUENCY: u32 = 21_477_272;

/// NTSC CPU divisor
///
/// Used to calculate the frequency of the CPU = `MASTER_FREQUENCY / CPU_PERIOD`.
pub const CPU_PERIOD: u8 = 12;

/// NTSC PPU divisor
///
/// Used to calculate the frequency of the PPU = `MASTER_FREQUENCY / PPU_PERIOD`.
pub const PPU_FREQUENCY: u8 = 4;

const CYCLE_BATCH_SIZE: u32 = 61015;
const CYCLE_BATCHES_PER_SECOND: u32 = MASTER_FREQUENCY / CYCLE_BATCH_SIZE;
const NANOS_PER_BATCH: u32 = 1_000_000_000 / CYCLE_BATCHES_PER_SECOND;

pub trait Processor<T: ReadAddr + WriteAddr> {
    fn cycle(&mut self, memory: &mut T);
}

pub struct Clock {
    batch: u32,
    next_batch: Instant,
}

impl Clock {
    pub fn new() -> Self {
        Clock {
            batch: 0,
            next_batch: Instant::now().add(Duration::new(0, NANOS_PER_BATCH)),
        }
    }

    pub fn cycle(&mut self) {
        if self.batch != CYCLE_BATCH_SIZE {
            self.batch += 1;
            return;
        }

        self.batch = 0;
        if self.next_batch > Instant::now() {
            let delay = self.next_batch.duration_since(Instant::now());
            thread::sleep(delay);
        }
        self.next_batch = self.next_batch.add(Duration::new(0, NANOS_PER_BATCH));
    }
}
