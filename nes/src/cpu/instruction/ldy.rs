use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Load index y with operand
///
/// Flags affected: N, Z
#[inline(always)]
fn ldy(core: &mut Core, operand: u8) {
  core.reg.y_idx = operand;
  core.reg.status.set_negative(core.reg.y_idx);
  core.reg.status.set_zero(core.reg.y_idx);
}

/// Load index y with memory
///
/// Flags affected: N, Z
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0xa0,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&ldy),
};

/// Load index y with memory
///
/// Flags affected: N, Z
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xa4,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Value(&ldy)),
};

/// Load index y with memory
///
/// Flags affected: N, Z
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0xb4,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Value(&ldy)),
};

/// Load index y with memory
///
/// Flags affected: N, Z
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xac,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&ldy)),
};

/// Load index y with memory
///
/// Flags affected: N, Z
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0xbc,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteX(Function::Value(&ldy)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};

  #[test]
  fn ldy_impl() {
    let mut core = Core::new(Registers::empty());
    let zero: u8 = 0b_0000_0000;
    let pos1: u8 = 0b_0000_0001;
    let neg1: u8 = 0b_1111_1111;

    ldy(&mut core, pos1);
    assert_eq!(core.reg.y_idx, pos1);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));

    ldy(&mut core, zero);
    assert_eq!(core.reg.y_idx, zero);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));

    ldy(&mut core, neg1);
    assert_eq!(core.reg.y_idx, neg1);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("LDY #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("LDY $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("LDY $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("LDY $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("LDY $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
