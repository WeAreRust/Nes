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
  operation: Operation::Zeropage(Function::Value(&ldx)),
};

/// Load index x with memory
///
/// Flags affected: N, Z
pub const ZERO_PAGE_Y: Instruction = Instruction {
  opcode: 0xb6,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageY(Function::Value(&ldx)),
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
    let negative28: u8 = -28i8 as u8;
    let positive42: u8 = 42;

    ldx(&mut core, positive42);
    assert_eq!(core.reg.x_idx, positive42);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));

    ldx(&mut core, 0);
    assert_eq!(core.reg.x_idx, 0);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));

    ldx(&mut core, negative28);
    assert_eq!(core.reg.x_idx, negative28);
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
