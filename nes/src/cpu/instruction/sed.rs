use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Set decimal flag
///
/// Flags affected: D
#[inline(always)]
fn sed(core: &mut Core) {
  // TODO: implementation
}

/// Set decimal flag
///
/// Flags affected: D
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xf8,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&sed),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn sed_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("SED")[0], IMPLIED.opcode);
  }
}
