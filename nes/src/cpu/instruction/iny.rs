use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Increment index y by one
///
/// Flags affected: N, Z
#[inline(always)]
fn iny(_core: &mut Core) {}

/// Increment index y by one
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xc8,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&iny),
};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("INY")[0], IMPLIED.opcode);
  }
}
