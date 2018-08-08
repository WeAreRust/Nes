use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Transfer accumulator to index y
///
/// Flags affected: N, Z
#[inline(always)]
fn tay(core: &mut Core) {
  // TODO: implementation
}

/// Transfer accumulator to index y
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xa8,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&tay),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn tay_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("TAY")[0], IMPLIED.opcode);
  }
}