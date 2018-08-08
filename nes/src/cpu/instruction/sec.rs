use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  register::StatusFlags,
  Core,
};

/// Set carry flag
///
/// Flags affected: C
#[inline(always)]
fn sec(core: &mut Core) {
  core.reg.status.set(StatusFlags::C_FLAG, true)
}

/// Set carry flag
///
/// Flags affected: C
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x38,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&sec),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn sec_impl() {
    let mut core = Core::new(Registers::empty());
    sec(&mut core);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("SEC")[0], IMPLIED.opcode);
  }
}
