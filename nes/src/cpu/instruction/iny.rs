use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Increment index y by one
///
/// Flags affected: N, Z
#[inline(always)]
fn iny(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.y_idx = core.reg.y_idx.wrapping_add(1);

  core.reg.status.set_zero(core.reg.y_idx);
  core.reg.status.set_negative(core.reg.y_idx);
}

/// Increment index y by one
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xc8,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&iny),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};
  use memory::block::BlockMemory;

  #[test]
  fn iny_impl() {
    let mut core = Core::new(Registers::empty());
    iny(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.y_idx, 1);
  }

  #[test]
  fn iny_impl_overflow() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 0xff;
    iny(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.y_idx, 0);
  }

  #[test]
  fn iny_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 0xff;
    iny(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.y_idx, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn iny_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 127;
    iny(&mut core, &mut BlockMemory::with_size(0));
    assert_eq!(core.reg.y_idx, 128);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("INY")[0], IMPLIED.opcode);
  }
}
