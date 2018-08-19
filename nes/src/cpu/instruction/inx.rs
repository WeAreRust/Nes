use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Increment index x by one
///
/// Flags affected: N, Z
#[inline(always)]
fn inx(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.x_idx = core.reg.x_idx.wrapping_add(1);

  core.reg.status.set_zero(core.reg.x_idx);
  core.reg.status.set_negative(core.reg.x_idx);
}

/// Increment index x by one
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xe8,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&inx),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};
  use memory::block::BlockMemory;

  #[test]
  fn inx_impl() {
    let mut core = Core::new(Registers::empty());
    inx(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.x_idx, 1);
  }

  #[test]
  fn inx_impl_overflow() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 0xff;
    inx(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.x_idx, 0);
  }

  #[test]
  fn inx_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 0xff;
    inx(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.x_idx, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn inx_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 127;
    inx(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.x_idx, 128);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("INX")[0], IMPLIED.opcode);
  }
}
