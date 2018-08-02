use cpu::{
  instruction::Instruction,
  operation::{Function, Operation},
  Core,
};

/// Clear decimal mode
///
/// Flags affected: D
#[inline(always)]
fn cld(core: &mut Core) {
  // TODO: implementation
}

/// Clear decimal mode
///
/// Flags affected: D
pub const IMPLIED: Instruction = Instruction {
  opcode: 0xd8,
  cycles: 2,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::Implied(&cld),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn cld_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CLD")[0], IMPLIED.opcode);
  }
}
