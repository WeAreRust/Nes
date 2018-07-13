use cpu::Core;
use memory::{Memory, ReadAddr};

impl Core {
    /// Jump absolute
    ///
    /// Flags affected: None
    pub fn jmp_absolute(&mut self, memory: &mut Memory) {
        self.reg.pc = self.absolute_addr(memory);
    }

    /// Jump indirect
    ///
    /// Flags affected: None
    ///
    /// An indirect jump must never use a vector beginning on the last byte of a page. If this
    /// occurs then the low byte should be as expected, and the high byte should wrap to the start
    /// of the page. See http://www.6502.org/tutorials/6502opcodes.html#JMP for details.
    ///
    /// TODO: Not sure if this wrap is being done?
    pub fn jmp_indirect(&mut self, memory: &mut Memory) {
        self.reg.pc = self.indirect_addr(memory);
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

    use cpu::{instruction, register::Registers};
    use memory::ReadAddr;

    #[test]
    fn jump_absolute() {
        let mut memory = Memory::with_bytes(nes_asm!("JMP $5597"));
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0x4c);
        assert_eq!(instruction::CYCLES[opcode as usize], 3);

        cpu.execute(opcode, &mut memory);
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
        assert_eq!(instruction::CYCLES[opcode as usize], 5);

        cpu.execute(opcode, &mut memory);
        assert_eq!(cpu.reg.pc, 0x5597);
    }

    #[test]
    fn indirect_address() {
        let mut memory = Memory::with_bytes(vec![0x03, 0x00, 0xff, 0x55, 0x97]);
        let mut cpu = Core::new(Registers::empty());

        let addr = cpu.indirect_addr(&mut memory);
        assert_eq!(addr, 0x5597);
    }

    #[test]
    fn indirect_address_overflow() {
        let mut bytes = vec![0; 256];
        bytes[0] = 0x97;
        bytes[255] = 0x55;

        let mut memory = Memory::with_bytes(bytes);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.pc = 255;

        let addr = cpu.indirect_addr(&mut memory);
        assert_eq!(addr, 0x5597);
    }
}
