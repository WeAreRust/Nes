use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};

/// Test bits in memory with accumulator
///
/// Flags affected: N, Z, V
#[inline(always)]
fn bit(core: &mut Core, operand: u8) {
  if operand & 0b1000_0000 > 0 {
    core.reg.status |= StatusFlags::N_FLAG;
  } else {
    core.reg.status &= !StatusFlags::N_FLAG;
  }

  if operand & 0b0100_0000 > 0 {
    core.reg.status |= StatusFlags::V_FLAG;
  } else {
    core.reg.status &= !StatusFlags::V_FLAG;
  }

  if operand & core.reg.acc == 0 {
    core.reg.status |= StatusFlags::Z_FLAG;
  } else {
    core.reg.status &= !StatusFlags::Z_FLAG;
  }
}

/// Test bits in memory with accumulator zero page
///
/// Flags affected: N, Z, V
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x24,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Value(&bit)),
};

/// Test bits in memory with accumulator absolute
///
/// Flags affected: N, Z, V
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x2C,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Value(&bit)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn bit_impl_zero() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0b0001_1111;
    bit(&mut core, 0b1100_0000);

    assert_eq!(
      core.reg.status,
      StatusFlags::N_FLAG | StatusFlags::V_FLAG | StatusFlags::Z_FLAG
    );
  }

  #[test]
  fn bit_impl_nonzero() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0b0001_1111;
    bit(&mut core, 0b1100_1000);

    assert_eq!(core.reg.status, StatusFlags::N_FLAG | StatusFlags::V_FLAG);
  }

  #[test]
  fn bit_impl_clear() {
    let mut core = Core::new(Registers::empty());
    core.reg.status =
      StatusFlags::N_FLAG | StatusFlags::V_FLAG | StatusFlags::Z_FLAG | StatusFlags::D_FLAG;

    core.reg.acc = 0b0001_1111;
    bit(&mut core, 0b0000_1000);

    assert_eq!(core.reg.status, StatusFlags::D_FLAG);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BIT $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("BIT $0000")[0], ABSOLUTE.opcode);
  }
}
