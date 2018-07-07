use self::register::Registers;

mod register;

/// Frequency of 6502 processor is ~1.79 MHz.
const FREQUENCY: usize = 1_789_773;

pub struct Core {
    pub reg: Registers,
}

impl Default for Core {
    fn default() -> Self {
        Core {
            reg: Registers::default(),
        }
    }
}
