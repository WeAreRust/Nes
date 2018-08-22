use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Set interrupt disable status
///
/// Flags affected: I
#[inline(always)]
fn sei(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.status.set(StatusFlags::I_FLAG, true)
}

/// Set interrupt disable status
///
/// Flags affected: I
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x78,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&sei),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn sei_impl() {
    let mut core = Core::new(Registers::empty());
    sei(&mut core, &mut BlockMemory::with_size(0));
    assert!(core.reg.status.contains(StatusFlags::I_FLAG));
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("SEI")[0], IMPLIED.opcode);
  }
}
