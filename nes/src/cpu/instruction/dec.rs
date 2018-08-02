use cpu::{
  instruction::Instruction,
  operation::{Function, Operation},
  Core,
};

/// Decrement memory by one
///
/// Flags affected: N, Z
#[inline(always)]
fn dec(core: &mut Core, address: u16) {
  // TODO: implementation
}

/// Decrement memory by one zero page
///
/// Flags affected: N, Z
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xc6,
  cycles: 5,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::Zeropage(Function::Address(&dec)),
};

/// Decrement memory by one zero page X
///
/// Flags affected: N, Z
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0xd6,
  cycles: 6,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::ZeropageX(Function::Address(&dec)),
};

/// Decrement memory by one absolute
///
/// Flags affected: N, Z
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xce,
  cycles: 3,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::Absolute(Function::Address(&dec)),
};

/// Decrement memory by one absolute X
///
/// Flags affected: N, Z
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0xde,
  cycles: 7,
  page_boundary_extra_cycle: true,
  page_branch_extra_cycles: false,
  operation: Operation::AbsoluteX(Function::Address(&dec)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn dec_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("DEC $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("DEC $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("DEC $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("DEC $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
