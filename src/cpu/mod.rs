use self::memory::{Memory, MemorySpace};
use self::register::Registers;

mod memory;
mod register;

/// Frequency of the 6502 processor is ~1.79 MHz.
const FREQUENCY: usize = 1_789_773;

pub struct Core {
    pub reg: Registers,
    pub memory: Memory,
}

impl Default for Core {
    fn default() -> Self {
        Core {
            reg: Registers::default(),
            memory: Memory::default(),
        }
    }
}

impl Core {
    pub fn with_data(data: MemorySpace) -> Self {
        Core {
            reg: Registers::default(),
            memory: Memory::with_data(data),
        }
    }
}
