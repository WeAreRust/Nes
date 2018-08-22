use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Clear carry flag
///
/// Flags affected: C
#[inline(always)]
fn clc(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.status.set(StatusFlags::C_FLAG, false)
}

/// Clear carry flag
///
/// Flags affected: C
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x18,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&clc),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn clc_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::C_FLAG, true);
    clc(&mut core, &mut BlockMemory::with_size(0));
    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CLC")[0], IMPLIED.opcode);
  }
}
