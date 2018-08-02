use cpu::{
  instruction::Instruction,
  operation::{Function, Operation},
  Core,
};

/// Shift operand left one bit
///
/// Flags affected: N, Z, C
#[inline(always)]
fn asl(core: &mut Core, operand: u8) {
  // TODO: implementation
}

/// Shift accumulator left one bit
///
/// Flags affected: N, Z, C
pub const ACCUMULATOR: Instruction = Instruction {
  opcode: 0x0a,
  cycles: 2,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::Accumulator(&asl),
};

/// Shift memory left one bit zero page
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x06,
  cycles: 5,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::Zeropage(Function::Value(&asl)),
};

/// Shift memory left one bit zero page X
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x16,
  cycles: 6,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::ZeropageX(Function::Value(&asl)),
};

/// Shift memory left one bit absolute
///
/// Flags affected: N, Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x0e,
  cycles: 6,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::Absolute(Function::Value(&asl)),
};

/// Shift memory left one bit absolute X
///
/// Flags affected: N, Z, C
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x1e,
  cycles: 7,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::AbsoluteX(Function::Value(&asl)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn asl_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("ASL A")[0], ACCUMULATOR.opcode);
    assert_eq!(nes_asm!("ASL $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("ASL $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("ASL $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("ASL $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
