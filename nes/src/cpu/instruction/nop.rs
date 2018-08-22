use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// No Operation
///
/// Flags affected: None
#[inline(always)]
fn nop(_core: &mut Core, _memory: &mut WriteAddr) {}

/// No Operation
///
/// Flags affected: None
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xea,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&nop),
};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("NOP")[0], IMPLIED.opcode);
  }
}
