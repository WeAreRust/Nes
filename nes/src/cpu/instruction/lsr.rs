use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Shift accumulator one bit right
///
/// Flags affected: Z, C
#[inline(always)]
fn lsr_acc(core: &mut Core, _operand: u8) {
  // TODO: implementation
}

/// Shift memory one bit right
///
/// Flags affected: Z, C
#[inline(always)]
fn lsr_memory(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  // TODO: implementation
}

/// Shift accumulator one bit right
///
/// Flags affected: Z, C
pub const ACCUMULATOR: Instruction = Instruction {
  opcode: 0x4a,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Accumulator(&lsr_acc),
};

/// Shift memory one bit right
///
/// Flags affected: Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x46,
  cycles: 5,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Address(&lsr_memory)),
};

/// Shift memory one bit right
///
/// Flags affected: Z, C
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x56,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Address(&lsr_memory)),
};

/// Shift memory one bit right
///
/// Flags affected: Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x4e,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&lsr_memory)),
};

/// Shift memory one bit right
///
/// Flags affected: Z, C
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x5e,
  cycles: 7,
  extra_cycle: ExtraCycle::None,
  operation: Operation::AbsoluteX(Function::Address(&lsr_memory)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn lsr_acc_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn lsr_memory_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("LSR A")[0], ACCUMULATOR.opcode);
    assert_eq!(nes_asm!("LSR $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("LSR $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("LSR $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("LSR $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
