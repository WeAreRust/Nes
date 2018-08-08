use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};

/// Add operand to accumulator with carry
///
/// Flags affected: N, Z, C, V
#[inline(always)]
fn adc(core: &mut Core, operand: u8) {
  let carry = u16::from(core.reg.status.contains(StatusFlags::C_FLAG));
  let sum = u16::from(core.reg.acc) + u16::from(operand) + carry;
  core.reg.acc = sum as u8; // Place the lo 8 bits into acc.

  core.reg.status.set_carry(sum);
  core.reg.status.set_overflow(sum);
  core.reg.status.set_zero(core.reg.acc);
  core.reg.status.set_negative(core.reg.acc);
}

/// Add memory to accumulator with carry immediate
///
/// Flags affected: N, Z, C, V
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0x69,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&adc),
};

/// Add memory to accumulator with carry zero page
///
/// Flags affected: N, Z, C, V
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x65,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Value(&adc)),
};

/// Add memory to accumulator with carry zero page X
///
/// Flags affected: N, Z, C, V
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x75,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Value(&adc)),
};

/// Add memory to accumulator with carry absolute
///
/// Flags affected: N, Z, C, V
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x6d,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&adc)),
};

/// Add memory to accumulator with carry absolute X
///
/// Flags affected: N, Z, C, V
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x7d,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteX(Function::Value(&adc)),
};

/// Add memory to accumulator with carry absolute Y
///
/// Flags affected: N, Z, C, V
pub const ABSOLUTE_Y: Instruction = Instruction {
  opcode: 0x79,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteY(Function::Value(&adc)),
};

/// Add memory to accumulator with carry indirect X
///
/// Flags affected: N, Z, C, V
pub const INDIRECT_X: Instruction = Instruction {
  opcode: 0x61,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::IndirectX(Function::Value(&adc)),
};

/// Add memory to accumulator with carry indirect Y
///
/// Flags affected: N, Z, C, V
pub const INDIRECT_Y: Instruction = Instruction {
  opcode: 0x71,
  cycles: 5,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::IndirectY(Function::Value(&adc)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};

  #[test]
  fn adc_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0x50;
    adc(&mut core, 0x90);
    assert_eq!(core.reg.acc, 0x50 + 0x90);
    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn adc_impl_add_carry() {
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::C_FLAG, true);
    core.reg.acc = 0x50;
    adc(&mut core, 0x90);
    assert_eq!(core.reg.acc, 0x50 + 0x90 + 0x01);
    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn adc_impl_carry_flag() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 255;
    adc(&mut core, 255);
    assert_eq!(core.reg.acc, 0b1111_1110);
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn adc_impl_zero_flag() {
    let mut core = Core::new(Registers::empty());
    adc(&mut core, 0);
    assert_eq!(core.reg.acc, 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn adc_impl_negative_flag() {
    let mut core = Core::new(Registers::empty());
    adc(&mut core, -12i8 as u8);
    assert_eq!(core.reg.acc, -12i8 as u8);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn adc_impl_overflow_flag_over() {
    let mut core = Core::new(Registers::empty());
    adc(&mut core, 128);
    assert_eq!(core.reg.acc, 128);
    assert!(core.reg.status.contains(StatusFlags::V_FLAG));
  }

  #[test]
  fn adc_impl_overflow_flag_under() {
    let mut core = Core::new(Registers::empty());
    adc(&mut core, -1i8 as u8);
    assert_eq!(core.reg.acc, -1i8 as u8);
    assert!(core.reg.status.contains(StatusFlags::V_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("ADC #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("ADC $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("ADC $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("ADC $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("ADC $0000,X")[0], ABSOLUTE_X.opcode);
    assert_eq!(nes_asm!("ADC $0000,Y")[0], ABSOLUTE_Y.opcode);
    assert_eq!(nes_asm!("ADC ($00,X)")[0], INDIRECT_X.opcode);
    assert_eq!(nes_asm!("ADC ($00),Y")[0], INDIRECT_Y.opcode);
  }
}
