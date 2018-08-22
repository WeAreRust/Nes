use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Clear overflow flag
///
/// Flags affected: V
#[inline(always)]
fn clv(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.status.set(StatusFlags::V_FLAG, false)
}

/// Clear overflow flag
///
/// Flags affected: V
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xB8,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&clv),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn clv_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::V_FLAG, true);
    clv(&mut core, &mut BlockMemory::with_size(0));
    assert!(!core.reg.status.contains(StatusFlags::V_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CLV")[0], IMPLIED.opcode);
  }
}
