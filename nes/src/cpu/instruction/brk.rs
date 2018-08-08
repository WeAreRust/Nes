use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Force break
///
/// Flags affected: I
#[inline(always)]
fn brk(core: &mut Core) {
  // TODO: implementation
}

/// Force break
///
/// Flags affected: I
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x00,
  cycles: 7,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&brk),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn brk_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BRK")[0], IMPLIED.opcode);
  }
}
