use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Push accumulator onto stack
///
/// Flags affected: None
#[inline(always)]
fn pha(core: &mut Core) {
  // TODO: implementation
}

/// Push accumulator onto stack
///
/// Flags affected: None
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x48,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&pha),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn pha_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("PHA")[0], IMPLIED.opcode);
  }
}
