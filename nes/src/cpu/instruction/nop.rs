use cpu::{instruction::Execute, Core};

/// No Operation
///
/// Flags affected: None
#[derive(Execute)]
#[opcode = 0xea]
#[cycles = 2]
pub struct Implicit;

#[inline(always)]
fn implicit<T>(_core: &mut Core, _memory: &mut T) {}

#[cfg(test)]
mod tests {
  use super::*;

  use cpu::{
    instruction::Instruction,
    register::{Registers, StatusFlags},
  };
  use memory::{block::BlockMemory, ReadAddr};

  #[test]
  fn nop() {
    let mut memory = BlockMemory::with_bytes(nes_asm!("NOP"));
    let mut core = Core::new(Registers::empty());

    let opcode = memory.read_addr(0);
    assert_eq!(opcode, <Implicit as Execute>::OPCODE);

    Instruction::from(opcode).execute(&mut core, &mut memory);
    assert_eq!(core.reg.status, StatusFlags::empty());
  }
}
