use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Exclusive-OR operand with accumulator
///
/// Flags affected: N, Z, C, V
#[inline(always)]
fn eor(core: &mut Core, operand: u8) {
  // TODO: implementation
}

/// Exclusive-OR memory with accumulator immediate
///
/// Flags affected: N, Z, C, V
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0x49,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&eor),
};

/// Exclusive-OR memory with accumulator zero page
///
/// Flags affected: N, Z, C, V
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x45,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Value(&eor)),
};

/// Exclusive-OR memory with accumulator zero page X
///
/// Flags affected: N, Z, C, V
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x55,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageX(Function::Value(&eor)),
};

/// Exclusive-OR memory with accumulator absolute
///
/// Flags affected: N, Z, C, V
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x4d,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&eor)),
};

/// Exclusive-OR memory with accumulator absolute X
///
/// Flags affected: N, Z, C, V
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x5d,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteX(Function::Value(&eor)),
};

/// Exclusive-OR memory with accumulator absolute Y
///
/// Flags affected: N, Z, C, V
pub const ABSOLUTE_Y: Instruction = Instruction {
  opcode: 0x59,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteY(Function::Value(&eor)),
};

/// Exclusive-OR memory with accumulator indirect X
///
/// Flags affected: N, Z, C, V
pub const INDIRECT_X: Instruction = Instruction {
  opcode: 0x41,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::IndirectX(Function::Value(&eor)),
};

/// Exclusive-OR memory with accumulator indirect Y
///
/// Flags affected: N, Z, C, V
pub const INDIRECT_Y: Instruction = Instruction {
  opcode: 0x51,
  cycles: 5,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::IndirectY(Function::Value(&eor)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn eor_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("EOR #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("EOR $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("EOR $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("EOR $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("EOR $0000,X")[0], ABSOLUTE_X.opcode);
    assert_eq!(nes_asm!("EOR $0000,Y")[0], ABSOLUTE_Y.opcode);
    assert_eq!(nes_asm!("EOR ($00,X)")[0], INDIRECT_X.opcode);
    assert_eq!(nes_asm!("EOR ($00),Y")[0], INDIRECT_Y.opcode);
  }
}
