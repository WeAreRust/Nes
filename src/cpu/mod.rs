use self::register::Registers;
use memory::{Memory, NesMemory, NesMemorySpace};

mod register;

/// Frequency of the 6502 processor is ~1.79 MHz.
const FREQUENCY: usize = 1_789_773;

pub struct Core {
    pub reg: Registers,
    pub memory: NesMemory,
}

impl Default for Core {
    fn default() -> Self {
        Core {
            reg: Registers::default(),
            memory: NesMemory::default(),
        }
    }
}

impl Core {
    pub fn with_data(data: NesMemorySpace) -> Self {
        Core {
            reg: Registers::default(),
            memory: NesMemory::with_data(data),
        }
    }
}
