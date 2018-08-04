use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Increment index x by one
///
/// Flags affected: N, Z
#[inline(always)]
fn inx(_core: &mut Core) {}

/// Increment index x by one
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xe8,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&inx),
};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("INX")[0], IMPLIED.opcode);
  }
}
