use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Transfer index x to stack register
///
/// Flags affected: None
#[inline(always)]
fn txs(core: &mut Core) {
  // TODO: implementation
}

/// Transfer index x to stack register
///
/// Flags affected: None
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x9a,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&txs),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn txs_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("TXS")[0], IMPLIED.opcode);
  }
}
