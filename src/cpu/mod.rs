use self::register::Registers;

use clock::Processor;
use memory::{Memory, ReadAddr};

mod instruction;
mod register;

pub struct Core {
    pub reg: Registers,
}

impl Processor for Core {
    fn cycle(&mut self, memory: &mut Memory) {
        let opcode = memory.read_addr(self.reg.pc);
        let _cycles = self.execute(opcode, memory);

        // TODO(joshleeb): Timing (use returned cycles).
    }
}

impl Core {
    pub fn new(reg: Registers) -> Self {
        Core { reg }
    }

    /// Value in memory.
    fn immediate_value(&mut self, memory: &mut Memory) -> u8 {
        let value = memory.read_addr(self.reg.pc);
        self.reg.pc += 1;

        value
    }

    /// Absolute address.
    fn absolute_addr(&mut self, memory: &mut Memory) -> u16 {
        let lo = memory.read_addr(self.reg.pc) as u16;
        let hi = memory.read_addr(self.reg.pc + 1) as u16;
        self.reg.pc += 2;

        lo | hi << 8
    }

    /// Indirect address.
    ///
    /// The 6502 processor has a bug in which only the high byte is incremented instead of the
    /// whole 16-bit address when computing the indirect address. See
    /// http://www.6502.org/tutorials/6502opcodes.html#JMP for details.
    fn indirect_addr(&mut self, memory: &mut Memory) -> u16 {
        let lo_addr = memory.read_addr(self.reg.pc) as u16;
        let hi_addr = memory.read_addr(self.reg.pc + 1) as u16;
        self.reg.pc += 2;

        let lo_adjusted = lo_addr + 1 | hi_addr << 8;
        let hi_adjusted = lo_addr | hi_addr << 8;

        let lo = memory.read_addr(lo_adjusted) as u16;
        let hi = memory.read_addr(hi_adjusted) as u16;
        lo | hi << 8
    }

    /// Execute the opcode and return the number of cycles.
    pub fn execute(&mut self, opcode: u8, memory: &mut Memory) -> usize {
        self.reg.pc += 1;

        match opcode {
            0x4c => self.jmp_absolute(memory),
            0x6c => self.jmp_indirect(memory),
            0xad => self.lda_absolute(memory),
            0xa9 => self.lda_immediate(memory),
            _ => unimplemented!(),
        }
    }
}
