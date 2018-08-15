use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Load index x with operand
///
/// Flags affected: N, Z
#[inline(always)]
fn ldx(core: &mut Core, operand: u8) {
  core.reg.x_idx = operand;
  core.reg.status.set_negative(core.reg.x_idx);
  core.reg.status.set_zero(core.reg.x_idx);
}

/// Load index x with memory
///
/// Flags affected: N, Z
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0xa2,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&ldx),
};

/// Load index x with memory
///
/// Flags affected: N, Z
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xa6,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Value(&ldx)),
};

/// Load index x with memory
///
/// Flags affected: N, Z
pub const ZERO_PAGE_Y: Instruction = Instruction {
  opcode: 0xb6,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageY(Function::Value(&ldx)),
};

/// Load index x with memory
///
/// Flags affected: N, Z
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xae,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&ldx)),
};

/// Load index x with memory
///
/// Flags affected: N, Z
pub const ABSOLUTE_Y: Instruction = Instruction {
  opcode: 0xbe,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteY(Function::Value(&ldx)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::register::StatusFlags;
  use cpu::Registers;

  #[test]
  fn ldx_impl() {
    let mut core = Core::new(Registers::empty());
    let zero: u8 = 0b_0000_0000;
    let pos1: u8 = 0b_0000_0001;
    let neg1: u8 = 0b_1111_1111;

    ldx(&mut core, pos1);
    assert_eq!(core.reg.x_idx, pos1);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));

    ldx(&mut core, zero);
    assert_eq!(core.reg.x_idx, zero);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));

    ldx(&mut core, neg1);
    assert_eq!(core.reg.x_idx, neg1);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("ldx #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("ldx $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("ldx $00,Y")[0], ZERO_PAGE_Y.opcode);
    assert_eq!(nes_asm!("ldx $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("ldx $0000,Y")[0], ABSOLUTE_Y.opcode);
  }
}
