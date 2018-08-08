use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Transfer accumulator to index x
///
/// Flags affected: N, Z
#[inline(always)]
fn tax(core: &mut Core) {
  // TODO: implementation
}

/// Transfer accumulator to index x
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xaa,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&tax),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn tax_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("TAX")[0], IMPLIED.opcode);
  }
}
