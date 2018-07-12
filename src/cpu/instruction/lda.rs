use cpu::Core;
use memory::{Memory, ReadAddr};

impl Core {
    /// Load the immediate value into the accumulator register (LDA)
    ///
    /// Flags affected: N, Z
    pub fn lda_imd(&mut self, memory: &mut Memory) -> usize {
        let value = self.imd_value(memory);
        self.reg.acc = value;

        // Update status flags.
        self.reg.status.set_negative(self.reg.acc);
        self.reg.status.set_zero(self.reg.acc);

        2
    }

    /// Load the value at the provided address into the accumulator register (LDA)
    ///
    /// Flags affected: N, Z
    pub fn lda_abs(&mut self, memory: &mut Memory) -> usize {
        let addr = self.abs_addr(memory);
        self.reg.pc += 1;
        self.reg.acc = memory.read_addr(addr);

        // Update status flags.
        self.reg.status.set_negative(self.reg.acc);
        self.reg.status.set_zero(self.reg.acc);

        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cpu::register::{Registers, StatusFlags};
    use memory::ReadAddr;

    #[test]
    fn load_accumulator_immediate() {
        let mut memory = Memory::with_bytes(nes_asm!("LDA #$5f"));
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0xa9);

        let cycles = cpu.execute(opcode, &mut memory);
        assert_eq!(cycles, 2);
        assert_eq!(cpu.reg.acc, 0x5f);
        assert_eq!(cpu.reg.status, StatusFlags::empty());
    }

    #[test]
    fn load_accumulator_immediate_zero_flag() {
        let mut memory = Memory::with_bytes(nes_asm!("LDA #$00"));
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0xa9);

        let cycles = cpu.execute(opcode, &mut memory);
        assert_eq!(cycles, 2);
        assert_eq!(cpu.reg.acc, 0x00);
        assert_eq!(cpu.reg.status, StatusFlags::Z_FLAG);
    }

    #[test]
    fn load_accumulator_immediate_negative_flag() {
        let mut memory = Memory::with_bytes(nes_asm!("LDA #$98"));
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0xa9);

        let cycles = cpu.execute(opcode, &mut memory);
        assert_eq!(cycles, 2);
        assert_eq!(cpu.reg.acc, 0b10011000);
        assert_eq!(cpu.reg.status, StatusFlags::N_FLAG);
    }

    #[test]
    fn load_accumulator_absolute() {
        let mut bytes = nes_asm!("LDA $0004");
        bytes.extend(vec![0xff, 0x44]);

        let mut memory = Memory::with_bytes(bytes);
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0xad);

        let cycles = cpu.execute(opcode, &mut memory);
        assert_eq!(cycles, 4);
        assert_eq!(cpu.reg.acc, 0x44);
        assert_eq!(cpu.reg.status, StatusFlags::empty());
    }
}
