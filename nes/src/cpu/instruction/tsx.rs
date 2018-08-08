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
  // TODO: implementation
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
  use cpu::Registers;

  #[test]
  fn tsx_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("TSX")[0], IMPLIED.opcode);
  }
}
