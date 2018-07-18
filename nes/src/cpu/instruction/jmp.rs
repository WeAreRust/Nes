use cpu::{self, Core};
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
    pub fn jmp_indirect(&mut self, memory: &mut Memory) {
        let arg_addr = self.absolute_addr(memory);
        self.reg.pc = self.indirect_addr(memory, arg_addr);
    }

    /// JMP is the only 6502 instruction to support indirection. The instruction contains a 16 bit
    /// address which identifies the location of the least significant byte of another 16 bit
    /// memory address which is the real target of the instruction.
    ///
    /// The 6502 process contains a bug specifically for indirect jumps that needs to be
    /// reproduced. If address $3000 contains $40, $30FF contains $80, and $3100 contains $50, the
    /// result of JMP ($30FF) will be a transfer of control to $4080 rather than $5080 as you
    /// intended i.e. the 6502 took the low byte of the address from $30FF and the high byte from
    /// $3000.
    fn indirect_addr(&mut self, memory: &mut Memory, arg_addr: u16) -> u16 {
        let lo_pc = arg_addr;
        let mut hi_pc = lo_pc + 1;

        let lo_page = lo_pc / cpu::PAGE_SIZE;
        let hi_page = hi_pc / cpu::PAGE_SIZE;
        if hi_page > lo_page {
            hi_pc = lo_page * cpu::PAGE_SIZE;
        }

        let lo = memory.read_addr(lo_pc) as u16;
        let hi = memory.read_addr(hi_pc) as u16;

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
        bytes.extend(vec![0xff, 0x97, 0x55]);

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
        let mut bytes = vec![0; 65536];
        bytes[0x30fe] = 0x80;
        bytes[0x30ff] = 0x50;

        let mut memory = Memory::with_bytes(bytes);
        let mut cpu = Core::new(Registers::empty());

        let addr = cpu.indirect_addr(&mut memory, 0x30fe);
        assert_eq!(addr, 0x5080);
    }

    #[test]
    fn indirect_address_overflow() {
        let mut bytes = vec![0; 65536];
        bytes[0x30ff] = 0x80;
        bytes[0x3100] = 0x50;
        bytes[0x3000] = 0x40;

        let mut memory = Memory::with_bytes(bytes);
        let mut cpu = Core::new(Registers::empty());

        let addr = cpu.indirect_addr(&mut memory, 0x30ff);
        assert_eq!(addr, 0x4080);
    }
}
