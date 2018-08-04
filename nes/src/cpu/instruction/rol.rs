use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Rotate accumulator one bit left
///
/// Flags affected: N, Z, C
#[inline(always)]
fn rol_acc(core: &mut Core, _operand: u8) {
  // TODO: implementation
}

/// Rotate memory one bit left
///
/// Flags affected: N, Z, C
#[inline(always)]
fn rol_memory(core: &mut Core, address: u16) {
  // TODO: implementation
}

/// Rotate accumulator one bit left
///
/// Flags affected: N, Z, C
pub const ACCUMULATOR: Instruction = Instruction {
  opcode: 0x2a,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Accumulator(&rol_acc),
};

/// Rotate memory one bit left
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x26,
  cycles: 5,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Address(&rol_memory)),
};

/// Rotate memory one bit left
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x36,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageX(Function::Address(&rol_memory)),
};

/// Rotate memory one bit left
///
/// Flags affected: N, Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x2e,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&rol_memory)),
};

/// Rotate memory one bit left
///
/// Flags affected: N, Z, C
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x3e,
  cycles: 7,
  extra_cycle: ExtraCycle::None,
  operation: Operation::AbsoluteX(Function::Address(&rol_memory)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn rol_acc_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn rol_memory_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("ROL A")[0], ACCUMULATOR.opcode);
    assert_eq!(nes_asm!("ROL $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("ROL $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("ROL $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("ROL $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
