use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Compare operand with accumulator
///
/// Flags affected: N, Z, C
#[inline(always)]
fn cmp(core: &mut Core, operand: u8) {
  // TODO: implementation
}

/// Compare memory with accumulator immediate
///
/// Flags affected: N, Z, C
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0xc9,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&cmp),
};

/// Compare memory with accumulator zero page
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xc5,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Value(&cmp)),
};

/// Compare memory with accumulator zero page X
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0xd5,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageX(Function::Value(&cmp)),
};

/// Compare memory with accumulator absolute
///
/// Flags affected: N, Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xcd,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&cmp)),
};

/// Compare memory with accumulator absolute X
///
/// Flags affected: N, Z, C
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0xdd,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteX(Function::Value(&cmp)),
};

/// Compare memory with accumulator absolute Y
///
/// Flags affected: N, Z, C
pub const ABSOLUTE_Y: Instruction = Instruction {
  opcode: 0xd9,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteY(Function::Value(&cmp)),
};

/// Compare memory with accumulator indirect X
///
/// Flags affected: N, Z, C
pub const INDIRECT_X: Instruction = Instruction {
  opcode: 0xc1,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::IndirectX(Function::Value(&cmp)),
};

/// Compare memory with accumulator indirect Y
///
/// Flags affected: N, Z, C
pub const INDIRECT_Y: Instruction = Instruction {
  opcode: 0xd1,
  cycles: 5,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::IndirectY(Function::Value(&cmp)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn cmp_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CMP #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("CMP $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("CMP $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("CMP $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("CMP $0000,X")[0], ABSOLUTE_X.opcode);
    assert_eq!(nes_asm!("CMP $0000,Y")[0], ABSOLUTE_Y.opcode);
    assert_eq!(nes_asm!("CMP ($00,X)")[0], INDIRECT_X.opcode);
    assert_eq!(nes_asm!("CMP ($00),Y")[0], INDIRECT_Y.opcode);
  }
}
