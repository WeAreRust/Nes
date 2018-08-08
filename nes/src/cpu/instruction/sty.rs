use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Store index y in memory
///
/// Flags affected: None
#[inline(always)]
fn sty(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  // TODO: implementation
}

/// Store index y in memory
///
/// Flags affected: None
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x84,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Address(&sty)),
};

/// Store index y in memory
///
/// Flags affected: None
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x94,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageX(Function::Address(&sty)),
};

/// Store index y in memory
///
/// Flags affected: None
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x8c,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&sty)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn sty_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("STY $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("STY $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("STY $0000")[0], ABSOLUTE.opcode);
  }
}
