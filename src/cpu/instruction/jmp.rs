use cpu::Core;
use memory::{Memory, ReadAddr};

impl Core {
    /// Jump to absolute address (JMP)
    ///
    /// Flags affected: None
    pub fn jmp_absolute(&mut self, memory: &mut Memory) -> usize {
        self.reg.pc = self.absolute_addr(memory);
        3
    }

    /// Jump to indirect address (JMP)
    ///
    /// Flags affected: None
    ///
    /// An indirect jump must never use a vector beginning on the last byte of a page. If this
    /// occurs then the low byte should be as expected, and the high byte should wrap to the start
    /// of the page. See http://www.6502.org/tutorials/6502opcodes.html#JMP for details.
    pub fn jmp_indirect(&mut self, memory: &mut Memory) -> usize {
        self.reg.pc = self.indirect_addr(memory);
        5
    }

    /// JMP is the only 6502 instruction to support indirection. The instruction contains a 16 bit
    /// address which identifies the location of the least significant byte of another 16 bit
    /// memory address which is the real target of the instruction.
    ///
    /// For example if location $0120 contains $FC and location $0121 contains $BA then the
    /// instruction JMP ($0120) will cause the next instruction execution to occur at $BAFC (e.g.
    /// the contents of $0120 and $0121).
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
}

#[cfg(test)]
mod tests {
    use super::*;

    use cpu::register::Registers;
    use memory::ReadAddr;

    #[test]
    fn jump_absolute() {
        let mut memory = Memory::with_bytes(nes_asm!("JMP $5597"));
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0x4c);

        let cycles = cpu.execute(opcode, &mut memory);
        assert_eq!(cycles, 3);
        assert_eq!(cpu.reg.pc, 0x5597);
    }

    #[test]
    fn jump_indirect() {
        let mut bytes = nes_asm!("JMP ($0004)");
        bytes.extend(vec![0xff, 0x55, 0x97]);

        let mut memory = Memory::with_bytes(bytes);
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0x6c);

        let cycles = cpu.execute(opcode, &mut memory);
        assert_eq!(cycles, 5);
        assert_eq!(cpu.reg.pc, 0x5597);
    }
}
