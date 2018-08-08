use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Load operand into accumulator
///
/// Flags affected: N, Z
fn lda(core: &mut Core, operand: u8) {
  core.reg.acc = operand;

  core.reg.status.set_negative(core.reg.acc);
  core.reg.status.set_zero(core.reg.acc);
}

/// Load accumulator immediate
///
/// Flags affected: N, Z
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0xa9,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&lda),
};

/// Load accumulator zero page
///
/// Flags affected: N, Z
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xa5,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Value(&lda)),
};

/// Load accumulator zero page X
///
/// Flags affected: N, Z
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0xb5,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageX(Function::Value(&lda)),
};

/// Load accumulator absolute
///
/// Flags affected: N, Z
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xad,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&lda)),
};

/// Load accumulator absolute X
///
/// Flags affected: N, Z
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0xbd,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteX(Function::Value(&lda)),
};

/// Load accumulator absolute Y
///
/// Flags affected: N, Z
pub const ABSOLUTE_Y: Instruction = Instruction {
  opcode: 0xb9,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteY(Function::Value(&lda)),
};

/// Load accumulator indirect X
///
/// Flags affected: N, Z
pub const INDIRECT_X: Instruction = Instruction {
  opcode: 0xa1,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::IndirectX(Function::Value(&lda)),
};

/// Load accumulator indirect Y
///
/// Flags affected: N, Z
pub const INDIRECT_Y: Instruction = Instruction {
  opcode: 0xb1,
  cycles: 2,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::IndirectY(Function::Value(&lda)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn lda_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0x00;
    lda(&mut core, 0x0F);
    assert_eq!(core.reg.acc, 0x0F);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("LDA #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("LDA $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("LDA $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("LDA $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("LDA $0000,X")[0], ABSOLUTE_X.opcode);
    assert_eq!(nes_asm!("LDA $0000,Y")[0], ABSOLUTE_Y.opcode);
    assert_eq!(nes_asm!("LDA ($00,X)")[0], INDIRECT_X.opcode);
    assert_eq!(nes_asm!("LDA ($00),Y")[0], INDIRECT_Y.opcode);
  }
}
