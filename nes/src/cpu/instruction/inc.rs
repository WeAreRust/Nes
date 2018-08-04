use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Inrement memory by one
///
/// Flags affected: N, Z
#[inline(always)]
fn inc(core: &mut Core, address: u16) {
  // TODO: implementation
}

/// Inrement memory by one
///
/// Flags affected: N, Z
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xe6,
  cycles: 5,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Zeropage(Function::Address(&inc)),
};

/// Inrement memory by one
///
/// Flags affected: N, Z
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0xf6,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageX(Function::Address(&inc)),
};

/// Inrement memory by one
///
/// Flags affected: N, Z
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xee,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&inc)),
};

/// Inrement memory by one
///
/// Flags affected: N, Z
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0xfe,
  cycles: 7,
  extra_cycle: ExtraCycle::None,
  operation: Operation::AbsoluteX(Function::Address(&inc)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn inc_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("INC $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("INC $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("INC $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("INC $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
