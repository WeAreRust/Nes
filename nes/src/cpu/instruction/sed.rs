use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Set decimal flag
///
/// Flags affected: D
#[inline(always)]
fn sed(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.status.set(StatusFlags::D_FLAG, true)
}

/// Set decimal flag
///
/// Flags affected: D
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xf8,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&sed),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn sed_impl() {
    let mut core = Core::new(Registers::empty());
    sed(&mut core, &mut BlockMemory::with_size(0));
    assert!(core.reg.status.contains(StatusFlags::D_FLAG));
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("SED")[0], IMPLIED.opcode);
  }
}
