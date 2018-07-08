use self::register::Registers;
use memory::{NesMemory, NesMemorySpace};

mod instruction;
mod register;

/// Frequency of the 6502 processor is ~1.79 MHz.
pub const FREQUENCY: usize = 1_789_773;

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

    pub fn execute(&mut self) {
        let opcode = self.memory.fetch(self.reg.pc);
        let cycles = instruction::execute(self, opcode);

        // TODO(joshleeb): Timing.
    }

    /// Absolute address.
    pub fn abs_addr(&mut self) -> u16 {
        let lo = self.memory.fetch(self.reg.pc) as u16;
        let hi = self.memory.fetch(self.reg.pc + 1) as u16;
        self.reg.pc += 2;

        lo | hi << 8
    }

    /// Indirect address.
    ///
    /// The 6502 processor has a bug in which only the high byte is incremented instead of the
    /// whole 16-bit address when computing the indirect address. See
    /// http://www.6502.org/tutorials/6502opcodes.html#JMP for details.
    pub fn indr_addr(&mut self) -> u16 {
        let lo_addr = self.memory.fetch(self.reg.pc) as u16;
        let hi_addr = self.memory.fetch(self.reg.pc + 1) as u16;
        self.reg.pc += 2;

        let lo_adjusted = lo_addr + 1 | hi_addr << 8;
        let hi_adjusted = lo_addr | hi_addr << 8;

        let lo = self.memory.fetch(lo_adjusted) as u16;
        let hi = self.memory.fetch(hi_adjusted) as u16;
        lo | hi << 8
    }
}
