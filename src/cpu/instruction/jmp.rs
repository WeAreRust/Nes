use cpu::Core;
use memory::Memory;

impl Core {
    /// Jump to absolute address (JMP)
    ///
    /// Flags affected: None
    pub fn jump_abs(&mut self, memory: &mut Memory) -> usize {
        self.reg.pc = self.abs_addr(memory);
        3
    }

    /// Jump to indirect address (JMP)
    ///
    /// Flags affected: None
    ///
    /// An indirect jump must never use a vector beginning on the last byte of a page. If this
    /// occurs then the low byte should be as expected, and the high byte should wrap to the start
    /// of the page. See http://www.6502.org/tutorials/6502opcodes.html#JMP for details.
    pub fn jump_indr(&mut self, memory: &mut Memory) -> usize {
        self.reg.pc = self.indr_addr(memory);
        5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use asm6502::assemble;
    use bytes::BytesMut;
    use memory::Memory;
    use cpu::register::Registers;

    #[test]
    fn jump_absolute() {
        let mut cpu = Core::new(Registers::empty());
        let mut memory = memory_from_asm("JMP $5597");

        let opcode = memory.fetch(0);
        assert_eq!(opcode, 0x4c);

        let cycles = cpu.execute(opcode, &mut memory);
        assert_eq!(cycles, 3);
        assert_eq!(cpu.reg.pc, 0x5597);
    }

    fn memory_from_asm(asm: &str) -> Memory {
        let mut buf = vec![];
        assemble(asm.as_bytes(), &mut buf).unwrap();
        Memory::with_bytes(BytesMut::from(buf))
    }
}
