use cpu::instruction::{Core, Instruction, Operation};

/// No Operation
///
/// Flags affected: None
#[inline(always)]
fn nop(_core: &mut Core) {}

/// No Operation
///
/// Flags affected: None
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xea,
  cycles: 2,
  page_boundary_extra_cycle: false,
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
