use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Transfer accumulator to index x
///
/// Flags affected: N, Z
#[inline(always)]
fn tax(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.x_idx = core.reg.acc;

  core.reg.status.set_negative(core.reg.x_idx);
  core.reg.status.set_zero(core.reg.x_idx);
}

/// Transfer accumulator to index x
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xaa,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&tax),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};
  use memory::block::BlockMemory;

  #[test]
  fn tax_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 1;
    tax(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.x_idx, 1);
  }

  #[test]
  fn tax_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    tax(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.x_idx, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn tax_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 128;
    tax(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.x_idx, 128);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("TAX")[0], IMPLIED.opcode);
  }
}
