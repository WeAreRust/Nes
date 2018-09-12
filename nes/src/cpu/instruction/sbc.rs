use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};

/// Subtract operand from accumulator with borrow
///
/// Flags affected: N, Z, C, V
#[inline(always)]
fn sbc(core: &mut Core, operand: u8) {
  let carry = u16::from(core.reg.status.contains(StatusFlags::C_FLAG));
  let result = u16::from(core.reg.acc) - u16::from(operand) - carry;
  core.reg.acc = result as u8; // Place the lo 8 bits into acc.

  core.reg.status.set_carry(result);
  core.reg.status.set_overflow(result);
  core.reg.status.set_zero(core.reg.acc);
  core.reg.status.set_negative(core.reg.acc);
}

/// Subtract memory from accumulator with borrow
///
/// Flags affected: N, Z, C, V
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0xe9,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&sbc),
};

/// Subtract memory from accumulator with borrow
///
/// Flags affected: N, Z, C, V
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xe5,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Value(&sbc)),
};

/// Subtract memory from accumulator with borrow
///
/// Flags affected: N, Z, C, V
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0xf5,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Value(&sbc)),
};

/// Subtract memory from accumulator with borrow
///
/// Flags affected: N, Z, C, V
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xed,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&sbc)),
};

/// Subtract memory from accumulator with borrow
///
/// Flags affected: N, Z, C, V
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0xfd,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteX(Function::Value(&sbc)),
};

/// Subtract memory from accumulator with borrow
///
/// Flags affected: N, Z, C, V
pub const ABSOLUTE_Y: Instruction = Instruction {
  opcode: 0xf9,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteY(Function::Value(&sbc)),
};

/// Subtract memory from accumulator with borrow
///
/// Flags affected: N, Z, C, V
pub const INDIRECT_X: Instruction = Instruction {
  opcode: 0xe1,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::IndirectX(Function::Value(&sbc)),
};

/// Subtract memory from accumulator with borrow
///
/// Flags affected: N, Z, C, V
pub const INDIRECT_Y: Instruction = Instruction {
  opcode: 0xf1,
  cycles: 5,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::IndirectY(Function::Value(&sbc)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn sbc_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 3;
    sbc(&mut core, 1);
    assert_eq!(core.reg.acc, 3 - 1);
    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn sbc_impl_sub_carry() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 37;
    sbc(&mut core, 18);
    assert_eq!(core.reg.acc, 37 - 18);
    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("SBC #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("SBC $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("SBC $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("SBC $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("SBC $0000,X")[0], ABSOLUTE_X.opcode);
    assert_eq!(nes_asm!("SBC $0000,Y")[0], ABSOLUTE_Y.opcode);
    assert_eq!(nes_asm!("SBC ($00,X)")[0], INDIRECT_X.opcode);
    assert_eq!(nes_asm!("SBC ($00),Y")[0], INDIRECT_Y.opcode);
  }
}
