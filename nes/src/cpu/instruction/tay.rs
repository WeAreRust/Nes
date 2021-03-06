use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Transfer accumulator to index y
///
/// Flags affected: N, Z
#[inline(always)]
fn tay(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.y_idx = core.reg.acc;

  core.reg.status.set_negative(core.reg.y_idx);
  core.reg.status.set_zero(core.reg.y_idx);
}

/// Transfer accumulator to index y
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xa8,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&tay),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};
  use memory::block::BlockMemory;

  #[test]
  fn tay_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 1;
    tay(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.y_idx, 1);
  }

  #[test]
  fn tay_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    tay(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.y_idx, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn tay_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 128;
    tay(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.y_idx, 128);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("TAY")[0], IMPLIED.opcode);
  }
}
