use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Rotate accumulator one bit right
///
/// Flags affected: N, Z, C
#[inline(always)]
fn ror_acc(core: &mut Core, _operand: u8) {
  // TODO: implementation
}

/// Rotate memory one bit right
///
/// Flags affected: N, Z, C
#[inline(always)]
fn ror_memory(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  // TODO: implementation
}

/// Rotate accumulator one bit right
///
/// Flags affected: N, Z, C
pub const ACCUMULATOR: Instruction = Instruction {
  opcode: 0x6a,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Accumulator(&ror_acc),
};

/// Rotate memory one bit right
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x66,
  cycles: 5,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Address(&ror_memory)),
};

/// Rotate memory one bit right
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x76,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageX(Function::Address(&ror_memory)),
};

/// Rotate memory one bit right
///
/// Flags affected: N, Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x6e,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&ror_memory)),
};

/// Rotate memory one bit right
///
/// Flags affected: N, Z, C
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x7e,
  cycles: 7,
  extra_cycle: ExtraCycle::None,
  operation: Operation::AbsoluteX(Function::Address(&ror_memory)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn ror_acc_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn ror_memory_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("ROR A")[0], ACCUMULATOR.opcode);
    assert_eq!(nes_asm!("ROR $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("ROR $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("ROR $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("ROR $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
