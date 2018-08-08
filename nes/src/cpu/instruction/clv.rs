use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Clear overflow flag
///
/// Flags affected: V
#[inline(always)]
fn clv(core: &mut Core) {
  // TODO: implementation
}

/// Clear overflow flag
///
/// Flags affected: V
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xB8,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&clv),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn clv_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CLV")[0], IMPLIED.opcode);
  }
}
