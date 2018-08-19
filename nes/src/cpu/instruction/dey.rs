use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Decrement indey y by one
///
/// Flags affected: N, Z
#[inline(always)]
fn dey(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.y_idx = core.reg.y_idx.wrapping_sub(1);

  core.reg.status.set_zero(core.reg.y_idx);
  core.reg.status.set_negative(core.reg.y_idx);
}

/// Decrement indey y by one
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x88,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&dey),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};
  use memory::block::BlockMemory;

  #[test]
  fn dey_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 0xff;
    dey(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.y_idx, 0xfe);
  }

  #[test]
  fn dey_impl_overflow() {
    let mut core = Core::new(Registers::empty());
    dey(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.y_idx, 0xff);
  }

  #[test]
  fn dey_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 1;
    dey(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.y_idx, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn dey_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 129;
    dey(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.y_idx, 128);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("DEY")[0], IMPLIED.opcode);
  }
}
