use cpu::Core;
use memory::{Memory, ReadAddr};

impl Core {
    /// Load accumulator immediate
    ///
    /// Flags affected: N, Z
    pub fn lda_immediate(&mut self, memory: &mut Memory) {
        let value = self.immediate_addr(memory);
        self.reg.acc = value;
        self.lda_update_status_flags();
    }

    /// Load accumulator zero page
    ///
    /// Flags affected: N, Z
    pub fn lda_zero_page(&mut self, memory: &mut Memory) {
        let addr = self.zero_page_addr(memory);
        self.reg.acc = memory.read_addr(addr);
        self.lda_update_status_flags();
    }

    /// Load accumulator zero page X
    ///
    /// Flags affected: N, Z
    pub fn lda_zero_page_x(&mut self, memory: &mut Memory) {
        let addr = self.zero_page_addr_x(memory);
        self.reg.acc = memory.read_addr(addr);
        self.lda_update_status_flags();
    }

    /// Load accumulator absolute
    ///
    /// Flags affected: N, Z
    pub fn lda_absolute(&mut self, memory: &mut Memory) {
        let addr = self.absolute_addr(memory);
        self.reg.acc = memory.read_addr(addr);
        self.lda_update_status_flags();
    }

    /// Load accumulator absolute X
    ///
    /// Flags affected: N, Z
    /// TODO: test
    pub fn lda_absolute_x(&mut self, memory: &mut Memory) {
        let addr = self.absolute_addr_x(memory);
        self.reg.acc = memory.read_addr(addr);
        self.lda_update_status_flags();
    }

    /// Load accumulator absolute Y
    ///
    /// Flags affected: N, Z
    /// TODO: test
    pub fn lda_absolute_y(&mut self, memory: &mut Memory) {
        let addr = self.absolute_addr_y(memory);
        self.reg.acc = memory.read_addr(addr);
        self.lda_update_status_flags();
    }

    /// Load accumulator indirect X
    ///
    /// Flags affected: N, Z
    /// TODO: test
    pub fn lda_indirect_x(&mut self, memory: &mut Memory) {
        let addr = self.idx_indirect(memory);
        self.reg.acc = memory.read_addr(addr);
        self.lda_update_status_flags();
    }

    /// Load accumulator indirect Y
    ///
    /// Flags affected: N, Z
    /// TODO: test
    pub fn lda_indirect_y(&mut self, memory: &mut Memory) {
        let addr = self.indirect_idx(memory);
        self.reg.acc = memory.read_addr(addr);
        self.lda_update_status_flags();
    }

    fn lda_update_status_flags(&mut self) {
        self.reg.status.set_negative(self.reg.acc);
        self.reg.status.set_zero(self.reg.acc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cpu::{
        instruction,
        register::{Registers, StatusFlags},
    };
    use memory::ReadAddr;

    #[test]
    fn load_accumulator_immediate() {
        let mut memory = Memory::with_bytes(nes_asm!("LDA #$5f"));
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0xa9);
        assert_eq!(instruction::CYCLES[opcode as usize], 2);

        cpu.execute(opcode, &mut memory);
        assert_eq!(cpu.reg.acc, 0x5f);
        assert_eq!(cpu.reg.status, StatusFlags::empty());
    }

    #[test]
    fn load_accumulator_zero_page() {
        let mut bytes = nes_asm!("LDA $03");
        bytes.extend(vec![0xff, 0x44]);

        let mut memory = Memory::with_bytes(bytes);
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0xa5);
        assert_eq!(instruction::CYCLES[opcode as usize], 3);

        cpu.execute(opcode, &mut memory);
        assert_eq!(cpu.reg.acc, 0x44);
        assert_eq!(cpu.reg.status, StatusFlags::empty());
    }

    #[test]
    fn load_accumulator_absolute() {
        let mut bytes = nes_asm!("LDA $0004");
        bytes.extend(vec![0xff, 0x44]);

        let mut memory = Memory::with_bytes(bytes);
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0xad);
        assert_eq!(instruction::CYCLES[opcode as usize], 4);

        cpu.execute(opcode, &mut memory);
        assert_eq!(cpu.reg.acc, 0x44);
        assert_eq!(cpu.reg.status, StatusFlags::empty());
    }

    #[test]
    fn load_accumulator_zero_flag() {
        let mut memory = Memory::with_bytes(nes_asm!("LDA #$00"));
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0xa9);
        assert_eq!(instruction::CYCLES[opcode as usize], 2);

        cpu.execute(opcode, &mut memory);
        assert_eq!(cpu.reg.acc, 0x00);
        assert_eq!(cpu.reg.status, StatusFlags::Z_FLAG);
    }

    #[test]
    fn load_accumulator_negative_flag() {
        let mut memory = Memory::with_bytes(nes_asm!("LDA #$98"));
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0xa9);
        assert_eq!(instruction::CYCLES[opcode as usize], 2);

        cpu.execute(opcode, &mut memory);
        assert_eq!(cpu.reg.acc, 0b10011000);
        assert_eq!(cpu.reg.status, StatusFlags::N_FLAG);
    }

}
