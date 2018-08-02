use cpu::{
  instruction::Instruction,
  operation::{Function, Operation},
  Core,
};

/// Decrement indey y by one
///
/// Flags affected: N, Z
#[inline(always)]
fn dey(core: &mut Core) {
  // TODO: implementation
}

/// Decrement indey y by one
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x88,
  cycles: 2,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::Implied(&dey),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn dey_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("DEY")[0], IMPLIED.opcode);
  }
}
