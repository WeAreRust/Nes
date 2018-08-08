use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Store index x in memory
///
/// Flags affected: None
#[inline(always)]
fn stx(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  // TODO: implementation
}

/// Store index x in memory
///
/// Flags affected: None
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x86,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Address(&stx)),
};

/// Store index x in memory
///
/// Flags affected: None
pub const ZERO_PAGE_Y: Instruction = Instruction {
  opcode: 0x96,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageY(Function::Address(&stx)),
};

/// Store index x in memory
///
/// Flags affected: None
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x8e,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&stx)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn stx_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("STX $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("STX $00,Y")[0], ZERO_PAGE_Y.opcode);
    assert_eq!(nes_asm!("STX $0000")[0], ABSOLUTE.opcode);
  }
}
