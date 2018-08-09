use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Decrement index x by one
///
/// Flags affected: N, Z
#[inline(always)]
fn dex(core: &mut Core) {
  core.reg.x_idx = core.reg.x_idx.wrapping_sub(1);

  core.reg.status.set_zero(core.reg.x_idx);
  core.reg.status.set_negative(core.reg.x_idx);
}

/// Decrement index x by one
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xca,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&dex),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};

  #[test]
  fn dex_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 0xff;
    dex(&mut core);
    assert_eq!(core.reg.x_idx, 0xfe);
  }

  #[test]
  fn dex_impl_overflow() {
    let mut core = Core::new(Registers::empty());
    dex(&mut core);
    assert_eq!(core.reg.x_idx, 0xff);
  }

  #[test]
  fn dex_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 1;
    dex(&mut core);
    assert_eq!(core.reg.x_idx, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn dex_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 129;
    dex(&mut core);
    assert_eq!(core.reg.x_idx, 128);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("DEX")[0], IMPLIED.opcode);
  }
}
