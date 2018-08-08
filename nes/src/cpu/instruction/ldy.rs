use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Load index y with operand
///
/// Flags affected: N, Z
#[inline(always)]
fn ldy(core: &mut Core, operand: u8) {
  // TODO: implementation
}

/// Load index y with memory
///
/// Flags affected: N, Z
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0xa0,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&ldy),
};

/// Load index y with memory
///
/// Flags affected: N, Z
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xa4,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Value(&ldy)),
};

/// Load index y with memory
///
/// Flags affected: N, Z
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0xb4,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageX(Function::Value(&ldy)),
};

/// Load index y with memory
///
/// Flags affected: N, Z
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xac,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&ldy)),
};

/// Load index y with memory
///
/// Flags affected: N, Z
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0xbc,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteX(Function::Value(&ldy)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn ldy_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("LDY #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("LDY $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("LDY $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("LDY $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("LDY $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
