use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Compare operand with index y
///
/// Flags affected: N, Z, C
#[inline(always)]
fn cpy(core: &mut Core, operand: u8) {
  // TODO: implementation
}

/// Compare memory with index y immediate
///
/// Flags affected: N, Z, C
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0xc0,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&cpy),
};

/// Compare memory with index y zero page
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xc4,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Value(&cpy)),
};

/// Compare memory with index y absolute
///
/// Flags affected: N, Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xcc,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&cpy)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn cpy_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CPY #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("CPY $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("CPY $0000")[0], ABSOLUTE.opcode);
  }
}
