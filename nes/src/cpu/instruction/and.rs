use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// AND operand with accumulator
///
/// Flags affected: N, Z
#[inline(always)]
fn and(core: &mut Core, operand: u8) {
  core.reg.acc &= operand;

  core.reg.status.set_negative(core.reg.acc);
  core.reg.status.set_zero(core.reg.acc);
}

/// AND memory with accumulator immediate
///
/// Flags affected: N, Z
pub const IMMEDIATE: Instruction = Instruction {
  opcode: 0x29,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Immediate(&and),
};

/// AND memory with accumulator zero page
///
/// Flags affected: N, Z
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x25,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Value(&and)),
};

/// AND memory with accumulator zero page X
///
/// Flags affected: N, Z
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x35,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Value(&and)),
};

/// AND memory with accumulator absolute
///
/// Flags affected: N, Z
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x2d,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&and)),
};

/// AND memory with accumulator absolute X
///
/// Flags affected: N, Z
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x3d,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteX(Function::Value(&and)),
};

/// AND memory with accumulator absolute Y
///
/// Flags affected: N, Z
pub const ABSOLUTE_Y: Instruction = Instruction {
  opcode: 0x39,
  cycles: 4,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteY(Function::Value(&and)),
};

/// AND memory with accumulator indirect X
///
/// Flags affected: N, Z
pub const INDIRECT_X: Instruction = Instruction {
  opcode: 0x21,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::IndirectX(Function::Value(&and)),
};

/// AND memory with accumulator indirect Y
///
/// Flags affected: N, Z
pub const INDIRECT_Y: Instruction = Instruction {
  opcode: 0x31,
  cycles: 5,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::IndirectY(Function::Value(&and)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn and_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0x55;
    and(&mut core, 0x0F);
    assert_eq!(core.reg.acc, 0x05);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("AND #$00")[0], IMMEDIATE.opcode);
    assert_eq!(nes_asm!("AND $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("AND $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("AND $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("AND $0000,X")[0], ABSOLUTE_X.opcode);
    assert_eq!(nes_asm!("AND $0000,Y")[0], ABSOLUTE_Y.opcode);
    assert_eq!(nes_asm!("AND ($00,X)")[0], INDIRECT_X.opcode);
    assert_eq!(nes_asm!("AND ($00),Y")[0], INDIRECT_Y.opcode);
  }
}
