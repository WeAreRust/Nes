use self::register::Registers;

use memory::Memory;

mod register;

/// Frequency of the 6502 processor (NTSC) is ~1.79 MHz.
pub const FREQUENCY: usize = 1_789_773;

pub struct Core {
    pub reg: Registers,
    pub memory: Memory,
}

impl Core {
    pub fn new(reg: Registers, memory: Memory) -> Self {
        Core { reg, memory }
    }

    pub fn fetch_execute(&mut self) {
        let opcode = self.memory.fetch(self.reg.pc);
        let _cycles = self.execute(opcode);
        // TODO(joshleeb): Timing (use cycles).
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

    /// Execute the opcode and return the number of cycles.
    pub fn execute(&mut self, opcode: u8) -> usize {
        self.reg.pc += 1;

        match opcode {
            0x4c => self.jump_abs(),
            0x6c => self.jump_indr(),
            _ => unimplemented!(),
        }
    }

    /// Jump to absolute address (JMP).
    ///
    /// Flags affected: None
    fn jump_abs(&mut self) -> usize {
        self.reg.pc = self.abs_addr();
        3
    }

    /// Jump to indirect address (JMP).
    ///
    /// Flags affected: None
    ///
    /// An indirect jump must never use a vector beginning on the last byte of a page. If this
    /// occurs then the low byte should be as expected, and the high byte should wrap to the start
    /// of the page. See http://www.6502.org/tutorials/6502opcodes.html#JMP for details.
    fn jump_indr(&mut self) -> usize {
        self.reg.pc = self.indr_addr();
        5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use asm6502::assemble;
    use bytes::BytesMut;
    use cpu::register::Registers;
    use memory::Memory;

    const TEST_MEMORY_SIZE: usize = 4096;

    #[test]
    fn jump_absolute() {
        let mut cpu = cpu_with_asm("JMP $5597");
        let opcode = cpu.memory.fetch(0);
        let cycles = cpu.execute(opcode);

        assert_eq!(cycles, 3);
        assert_eq!(opcode, 0x4c);
        assert_eq!(cpu.reg.pc, 0x5597);
    }

    fn cpu_with_asm(asm: &str) -> Core {
        let asm = asm.as_bytes();
        let mut buf = vec![];
        assemble(asm, &mut buf).unwrap();

        let memory = Memory::with_bytes(BytesMut::from(buf));
        Core::new(Registers::empty(), memory)
    }
}
