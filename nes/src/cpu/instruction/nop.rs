use cpu::Core;

impl Core {
    /// No Operation
    ///
    /// Flags affected: None
    pub fn nop(&self) {
      // Nothing to do
      let _x = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cpu::{
        instruction,
        register::{Registers, StatusFlags},
    };
    use memory::{Memory, ReadAddr};

    #[test]
    fn nop() {
        let mut memory = Memory::with_bytes(nes_asm!("NOP"));
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, 0xea);
        assert_eq!(instruction::CYCLES[opcode as usize], 2);

        cpu.execute(opcode, &mut memory);
        assert_eq!(cpu.reg.status, StatusFlags::empty());
    }
}