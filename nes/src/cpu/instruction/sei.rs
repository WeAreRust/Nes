use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Set interrupt disable status
///
/// Flags affected: I
#[inline(always)]
fn sei(core: &mut Core) {
  // TODO: implementation
}

/// Set interrupt disable status
///
/// Flags affected: I
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x78,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&sei),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn sei_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("SEI")[0], IMPLIED.opcode);
  }
}
