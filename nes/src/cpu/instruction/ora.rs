use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// OR operand with accumulator
///
/// Flags affected: N, Z
#[inline(always)]
fn ora(core: &mut Core, operand: u8) {
  core.reg.acc |= operand;

  core.reg.status.set_negative(core.reg.acc);
  core.reg.status.set_zero(core.reg.acc);
}

/// OR memory with accumulator immediate
///
/// Flags affected: N, Z
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0x09,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&ora),
};

/// OR memory with accumulator zero page
///
/// Flags affected: N, Z
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x05,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Value(&ora)),
};

/// OR memory with accumulator zero page X
///
/// Flags affected: N, Z
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x15,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Value(&ora)),
};

/// OR memory with accumulator absolute
///
/// Flags affected: N, Z
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x0d,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&ora)),
};

/// OR memory with accumulator absolute X
///
/// Flags affected: N, Z
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x1d,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteX(Function::Value(&ora)),
};

/// OR memory with accumulator absolute Y
///
/// Flags affected: N, Z
pub const ABSOLUTE_Y: Instruction = Instruction {
  opcode: 0x19,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteY(Function::Value(&ora)),
};

/// OR memory with accumulator indirect X
///
/// Flags affected: N, Z
pub const INDIRECT_X: Instruction = Instruction {
  opcode: 0x01,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::IndirectX(Function::Value(&ora)),
};

/// OR memory with accumulator indirect Y
///
/// Flags affected: N, Z
pub const INDIRECT_Y: Instruction = Instruction {
  opcode: 0x11,
  cycles: 5,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::IndirectY(Function::Value(&ora)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn ora_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0b0000_1111;
    ora(&mut core, 0b0101_0101);
    assert_eq!(core.reg.acc, 0b0101_1111);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("ORA #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("ORA $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("ORA $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("ORA $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("ORA $0000,X")[0], ABSOLUTE_X.opcode);
    assert_eq!(nes_asm!("ORA $0000,Y")[0], ABSOLUTE_Y.opcode);
    assert_eq!(nes_asm!("ORA ($00,X)")[0], INDIRECT_X.opcode);
    assert_eq!(nes_asm!("ORA ($00),Y")[0], INDIRECT_Y.opcode);
  }
}
