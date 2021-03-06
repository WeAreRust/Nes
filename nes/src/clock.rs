use std::ops::Add;
use std::thread;
use std::time::{Duration, Instant};

/// NTSC master clock frequency (per second)
pub const MASTER_FREQUENCY: u32 = 21_477_272;

/// NTSC CPU divisor
///
/// Used to calculate the frequency of the CPU = `MASTER_FREQUENCY / CPU_PERIOD`.
pub const CPU_PERIOD: u8 = 12;

/// NTSC PPU divisor
///
/// Used to calculate the frequency of the PPU = `MASTER_FREQUENCY / PPU_PERIOD`.
pub const PPU_PERIOD: u8 = 4;

// We want a value that will not be noticable to the human eye (> 24/sec),
// will not round down to zero in sleep (< 1000/sec),
// and is close to a factor of MASTER_FREQUENCY.
// 352 * 61_015 = 21_477_272 + 8
const BATCHES_PER_SECOND: u32 = 352;
const CYCLE_BATCH_SIZE: u32 = MASTER_FREQUENCY / BATCHES_PER_SECOND;
const NANOS_PER_BATCH: u32 = 1_000_000_000 / BATCHES_PER_SECOND;

pub struct Clock {
  batch: u32,
  next_batch: Instant,
}

impl Default for Clock {
  fn default() -> Self {
    Clock {
      batch: 0,
      next_batch: Instant::now().add(Duration::new(0, NANOS_PER_BATCH)),
    }
  }
}

impl Clock {
  pub fn new() -> Self {
    Clock::default()
  }

  pub fn cycle(&mut self) {
    if self.batch != CYCLE_BATCH_SIZE {
      self.batch += 1;
      return;
    }

    self.batch = 0;
    let present = Instant::now();
    if self.next_batch > present {
      let delay = self.next_batch.duration_since(present);
      thread::sleep(delay);
    }
    self.next_batch = self.next_batch.add(Duration::new(0, NANOS_PER_BATCH));
  }
}
