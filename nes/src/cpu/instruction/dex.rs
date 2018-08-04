use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Decrement index x by one
///
/// Flags affected: N, Z
#[inline(always)]
fn dex(core: &mut Core) {
  // TODO: implementation
}

/// Decrement index x by one
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xca,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&dex),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn dex_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("DEX")[0], IMPLIED.opcode);
  }
}
