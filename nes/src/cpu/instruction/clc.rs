use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Clear carry flag
///
/// Flags affected: C
#[inline(always)]
fn clc(core: &mut Core) {
  // TODO: implementation
}

/// Clear carry flag
///
/// Flags affected: C
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x18,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&clc),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn clc_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CLC")[0], IMPLIED.opcode);
  }
}
