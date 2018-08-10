use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};

/// Compare operand with accumulator
///
/// Flags affected: N, Z, C
#[inline(always)]
fn cmp(core: &mut Core, operand: u8) {
  let acc = core.reg.acc as i8;
  let operand = operand as i8;

  core.reg.status.set(StatusFlags::C_FLAG, acc >= operand);
  core.reg.status.set_negative(core.reg.acc);
  core.reg.status.set_zero(core.reg.acc);
}

/// Compare memory with accumulator immediate
///
/// Flags affected: N, Z, C
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0xc9,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&cmp),
};

/// Compare memory with accumulator zero page
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xc5,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Value(&cmp)),
};

/// Compare memory with accumulator zero page X
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0xd5,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Value(&cmp)),
};

/// Compare memory with accumulator absolute
///
/// Flags affected: N, Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xcd,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&cmp)),
};

/// Compare memory with accumulator absolute X
///
/// Flags affected: N, Z, C
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0xdd,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteX(Function::Value(&cmp)),
};

/// Compare memory with accumulator absolute Y
///
/// Flags affected: N, Z, C
pub const ABSOLUTE_Y: Instruction = Instruction {
  opcode: 0xd9,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteY(Function::Value(&cmp)),
};

/// Compare memory with accumulator indirect X
///
/// Flags affected: N, Z, C
pub const INDIRECT_X: Instruction = Instruction {
  opcode: 0xc1,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::IndirectX(Function::Value(&cmp)),
};

/// Compare memory with accumulator indirect Y
///
/// Flags affected: N, Z, C
pub const INDIRECT_Y: Instruction = Instruction {
  opcode: 0xd1,
  cycles: 5,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::IndirectY(Function::Value(&cmp)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn cmp_impl_eq() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 2;
    cmp(&mut core, 2);
    assert_eq!(core.reg.acc, 2);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cmp_impl_neg_eq() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = -1i8 as u8;
    cmp(&mut core, -1i8 as u8);
    assert_eq!(core.reg.acc, -1i8 as u8);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cmp_impl_gt() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 2;
    cmp(&mut core, 1);
    assert_eq!(core.reg.acc, 2);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cmp_impl_gt_neg() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0;
    cmp(&mut core, -2i8 as u8);
    assert_eq!(core.reg.acc, 0);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn cmp_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0;
    cmp(&mut core, 0);
    assert_eq!(core.reg.acc, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn cmp_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = -1i8 as u8;
    cmp(&mut core, 0);
    assert_eq!(core.reg.acc, -1i8 as u8);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CMP #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("CMP $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("CMP $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("CMP $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("CMP $0000,X")[0], ABSOLUTE_X.opcode);
    assert_eq!(nes_asm!("CMP $0000,Y")[0], ABSOLUTE_Y.opcode);
    assert_eq!(nes_asm!("CMP ($00,X)")[0], INDIRECT_X.opcode);
    assert_eq!(nes_asm!("CMP ($00),Y")[0], INDIRECT_Y.opcode);
  }
}
