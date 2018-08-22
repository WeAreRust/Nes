use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Clear decimal mode
///
/// Flags affected: D
#[inline(always)]
fn cld(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.status.set(StatusFlags::D_FLAG, false)
}

/// Clear decimal mode
///
/// Flags affected: D
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xd8,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&cld),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn cld_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::D_FLAG, true);
    cld(&mut core, &mut BlockMemory::with_size(0));
    assert!(!core.reg.status.contains(StatusFlags::D_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CLD")[0], IMPLIED.opcode);
  }
}
