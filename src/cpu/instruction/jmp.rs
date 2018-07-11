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
    use cpu::register::Registers;
    use memory::{Memory, ReadAddr};

    #[test]
    fn jump_absolute() {
        let mut bytes = nes_asm!("JMP $5597");

        let mut memory = Memory::with_bytes(bytes);
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

    fn memory_from_asm(asm: &str) -> Memory {
        let mut buf = vec![];
        assemble(asm.as_bytes(), &mut buf).unwrap();
        println!("{:x?}", buf);
        Memory::with_bytes(BytesMut::from(buf))
    }
}
