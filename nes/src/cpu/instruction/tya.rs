use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Transfer index y to accumulator
///
/// Flags affected: N, Z
#[inline(always)]
fn tya(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.acc = core.reg.y_idx;

  core.reg.status.set_negative(core.reg.acc);
  core.reg.status.set_zero(core.reg.acc);
}

/// Transfer index y to accumulator
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x98,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&tya),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};
  use memory::block::BlockMemory;

  #[test]
  fn tya_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 1;
    tya(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.acc, 1);
  }

  #[test]
  fn tya_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    tya(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.acc, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn tya_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 128;
    tya(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.acc, 128);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("TYA")[0], IMPLIED.opcode);
  }
}
