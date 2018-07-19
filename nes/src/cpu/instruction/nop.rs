use cpu::{instruction::Instruction, Core};
use memory::Memory;

/// No Operation
///
/// Flags affected: None
#[derive(Instruction)]
#[opcode = 0xea]
#[cycles = 2]
pub struct Implicit;

fn implicit(_core: &mut Core, _memory: &mut Memory) {}

#[cfg(test)]
mod tests {
    use super::*;

    use cpu::{
        instruction,
        register::{Registers, StatusFlags},
    };
    use memory::{Memory, ReadAddr};

    // TODO: Test needs to be fixed (PC might be wrong?).
    #[test]
    #[ignore]
    fn nop() {
        let mut memory = Memory::with_bytes(nes_asm!("NOP"));
        let mut cpu = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, <Implicit as Instruction>::OPCODE);

        instruction::execute(opcode, &mut cpu, &mut memory);
        assert_eq!(cpu.reg.status, StatusFlags::empty());
        assert_eq!(cpu.reg.pc, 2);
    }
}
