use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Clear interrupt disable bit
///
/// Flags affected: I
#[inline(always)]
fn cli(core: &mut Core) {
  // TODO: implementation
}

/// Clear interrupt disable bit
///
/// Flags affected: I
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x58,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&cli),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn cli_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CLI")[0], IMPLIED.opcode);
  }
}
