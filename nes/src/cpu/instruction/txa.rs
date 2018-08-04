use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Transfer index x to accumulator
///
/// Flags affected: N, Z
#[inline(always)]
fn txa(core: &mut Core) {
  // TODO: implementation
}

/// Transfer index x to accumulator
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x8a,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&txa),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn txa_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("TXA")[0], IMPLIED.opcode);
  }
}
