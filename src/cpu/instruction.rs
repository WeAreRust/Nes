use cpu::Core;

impl Core {
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
