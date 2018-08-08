use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Compare operand with index x
///
/// Flags affected: N, Z, C
#[inline(always)]
fn cpx(core: &mut Core, operand: u8) {
  // TODO: implementation
}

/// Compare memory with index x immediate
///
/// Flags affected: N, Z, C
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0xe0,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&cpx),
};

/// Compare memory with index x zero page
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xe4,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Value(&cpx)),
};

/// Compare memory with index x absolute
///
/// Flags affected: N, Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xec,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&cpx)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn cpx_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CPX #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("CPX $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("CPX $0000")[0], ABSOLUTE.opcode);
  }
}
