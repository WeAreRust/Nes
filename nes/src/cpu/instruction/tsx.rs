use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Transfer stack pointer to index x
///
/// Flags affected: N, Z
#[inline(always)]
fn tsx(core: &mut Core) {
  core.reg.x_idx = core.reg.stack;

  core.reg.status.set_zero(core.reg.x_idx);
  core.reg.status.set_negative(core.reg.x_idx);
}

/// Transfer stack pointer to index x
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xba,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&tsx),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};

  #[test]
  fn tsx_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.stack = 1;
    tsx(&mut core);
    assert_eq!(core.reg.x_idx, 1);
  }

  #[test]
  fn tsx_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    tsx(&mut core);
    assert_eq!(core.reg.x_idx, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn tsx_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.stack = 128;
    tsx(&mut core);
    assert_eq!(core.reg.x_idx, 128);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("TSX")[0], IMPLIED.opcode);
  }
}
