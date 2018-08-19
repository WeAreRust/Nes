use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Pull processor status from stack
///
/// Flags affected: All
#[inline(always)]
fn plp(_core: &mut Core, _memory: &mut WriteAddr) {
  // TODO: implementation
}

/// Pull processor status from stack
///
/// Flags affected: All
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x28,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&plp),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn plp_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("PLP")[0], IMPLIED.opcode);
  }
}
