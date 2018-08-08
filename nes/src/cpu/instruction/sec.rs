use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Set carry flag
///
/// Flags affected: C
#[inline(always)]
fn sec(core: &mut Core) {
  // TODO: implementation
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
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("SEC")[0], IMPLIED.opcode);
  }
}
