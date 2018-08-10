use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};

/// Compare operand with index x
///
/// Flags affected: N, Z, C
#[inline(always)]
fn cpx(core: &mut Core, operand: u8) {
  let x_idx = core.reg.x_idx as i8;
  let operand = operand as i8;

  core.reg.status.set(StatusFlags::C_FLAG, x_idx >= operand);
  core.reg.status.set_negative(core.reg.x_idx);
  core.reg.status.set_zero(core.reg.x_idx);
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
  operation: Operation::ZeroPage(Function::Value(&cpx)),
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
  fn cpx_impl_eq() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 2;
    cpx(&mut core, 2);
    assert_eq!(core.reg.x_idx, 2);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cpx_impl_neg_eq() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = -1i8 as u8;
    cpx(&mut core, -1i8 as u8);
    assert_eq!(core.reg.x_idx, -1i8 as u8);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cpx_impl_gt() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 2;
    cpx(&mut core, 1);
    assert_eq!(core.reg.x_idx, 2);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cpx_impl_gt_neg() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 0;
    cpx(&mut core, -2i8 as u8);
    assert_eq!(core.reg.x_idx, 0);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cpx_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 0;
    cpx(&mut core, 0);
    assert_eq!(core.reg.x_idx, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn cpx_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = -1i8 as u8;
    cpx(&mut core, 0);
    assert_eq!(core.reg.x_idx, -1i8 as u8);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CPX #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("CPX $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("CPX $0000")[0], ABSOLUTE.opcode);
  }
}
