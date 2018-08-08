use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Pull accumulator from stack
///
/// Flags affected: N, Z
#[inline(always)]
fn pla(core: &mut Core) {
  // TODO: implementation
}

/// Pull accumulator from stack
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x68,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&pla),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn pla_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("PLA")[0], IMPLIED.opcode);
  }
}
