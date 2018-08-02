use cpu::{
  instruction::Instruction,
  operation::{Function, Operation},
  Core,
};

/// Test bits in memory with accumulator
///
/// Flags affected: N, Z, V
#[inline(always)]
fn bit(core: &mut Core, operand: u8) {
  // TODO: implementation
  unimplemented!();
}

/// Test bits in memory with accumulator zero page
///
/// Flags affected: N, Z, V
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x24,
  cycles: 3,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::Zeropage(Function::Value(&bit)),
};

/// Test bits in memory with accumulator absolute
///
/// Flags affected: N, Z, V
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x2C,
  cycles: 4,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::Absolute(Function::Value(&bit)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn beq_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BIT $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("BIT $0000")[0], ABSOLUTE.opcode);
  }
}
