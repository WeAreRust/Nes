use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};

/// Compare operand with index y
///
/// Flags affected: N, Z, C
#[inline(always)]
fn cpy(core: &mut Core, operand: u8) {
  let y_idx = core.reg.y_idx as i8;
  let operand = operand as i8;

  core.reg.status.set(StatusFlags::C_FLAG, y_idx >= operand);
  core.reg.status.set_negative(core.reg.y_idx);
  core.reg.status.set_zero(core.reg.y_idx);
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
  operation: Operation::ZeroPage(Function::Value(&cpy)),
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
  fn cpy_impl_eq() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 2;
    cpy(&mut core, 2);
    assert_eq!(core.reg.y_idx, 2);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cpy_impl_neg_eq() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = -1i8 as u8;
    cpy(&mut core, -1i8 as u8);
    assert_eq!(core.reg.y_idx, -1i8 as u8);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cpy_impl_gt() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 2;
    cpy(&mut core, 1);
    assert_eq!(core.reg.y_idx, 2);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cpy_impl_gt_neg() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 0;
    cpy(&mut core, -2i8 as u8);
    assert_eq!(core.reg.y_idx, 0);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cpy_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 0;
    cpy(&mut core, 0);
    assert_eq!(core.reg.y_idx, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn cpy_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = -1i8 as u8;
    cpy(&mut core, 0);
    assert_eq!(core.reg.y_idx, -1i8 as u8);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CPY #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("CPY $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("CPY $0000")[0], ABSOLUTE.opcode);
  }
}
