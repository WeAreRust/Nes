use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Store accumulator in memory
///
/// Flags affected: None
#[inline(always)]
fn sta(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  // TODO: implementation
}

/// Store accumulator in memory
///
/// Flags affected: None
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x85,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Address(&sta)),
};

/// Store accumulator in memory
///
/// Flags affected: None
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x95,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageX(Function::Address(&sta)),
};

/// Store accumulator in memory
///
/// Flags affected: None
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x8d,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&sta)),
};

/// Store accumulator in memory
///
/// Flags affected: None
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x9d,
  cycles: 5,
  extra_cycle: ExtraCycle::None,
  operation: Operation::AbsoluteX(Function::Address(&sta)),
};

/// Store accumulator in memory
///
/// Flags affected: None
pub const ABSOLUTE_Y: Instruction = Instruction {
  opcode: 0x99,
  cycles: 5,
  extra_cycle: ExtraCycle::None,
  operation: Operation::AbsoluteY(Function::Address(&sta)),
};

/// Store accumulator in memory
///
/// Flags affected: None
pub const INDIRECT_X: Instruction = Instruction {
  opcode: 0x81,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::IndirectX(Function::Address(&sta)),
};

/// Store accumulator in memory
///
/// Flags affected: None
pub const INDIRECT_Y: Instruction = Instruction {
  opcode: 0x91,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::IndirectY(Function::Address(&sta)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn sta_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("STA $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("STA $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("STA $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("STA $0000,X")[0], ABSOLUTE_X.opcode);
    assert_eq!(nes_asm!("STA $0000,Y")[0], ABSOLUTE_Y.opcode);
    assert_eq!(nes_asm!("STA ($00,X)")[0], INDIRECT_X.opcode);
    assert_eq!(nes_asm!("STA ($00),Y")[0], INDIRECT_Y.opcode);
  }
}