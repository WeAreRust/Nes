use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Add operand to accumulator with carry
///
/// Flags affected: N, Z, C, V
#[inline(always)]
fn adc(core: &mut Core, operand: u8) {
  // TODO: implementation
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
  operation: Operation::Zeropage(Function::Value(&adc)),
};

/// Add memory to accumulator with carry zero page X
///
/// Flags affected: N, Z, C, V
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x75,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeropageX(Function::Value(&adc)),
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
  use cpu::Registers;

  #[test]
  fn adc_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
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
