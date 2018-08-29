use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

// Rotate operand one bit right
///
/// Flags affected: N, Z, C
#[inline(always)]
fn rotate_right(core: &mut Core, operand: u8) -> u8 {
  let hi: u8 = if core.reg.status.contains(StatusFlags::C_FLAG) {
    0b1000_0000
  } else {
    0
  };

  let result = hi | (operand >> 1);

  // Move operand bit 0 into bit for carry test
  core.reg.status.set_carry(u16::from(operand) << 8);
  core.reg.status.set_negative(result);
  core.reg.status.set_zero(result);

  result
}

/// Rotate accumulator one bit right
///
/// Flags affected: N, Z, C
#[inline(always)]
fn ror_acc(core: &mut Core, _operand: u8) {
  core.reg.acc = rotate_right(core, core.reg.acc);
}

/// Rotate memory one bit right
///
/// Flags affected: N, Z, C
#[inline(always)]
fn ror_mem(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  let result = rotate_right(core, memory.read_addr(address));
  memory.write_addr(address, result);
}

/// Rotate accumulator one bit right
///
/// Flags affected: N, Z, C
pub const ACCUMULATOR: Instruction = Instruction {
  opcode: 0x6a,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Accumulator(&ror_acc),
};

/// Rotate memory one bit right
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x66,
  cycles: 5,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Address(&ror_mem)),
};

/// Rotate memory one bit right
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x76,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Address(&ror_mem)),
};

/// Rotate memory one bit right
///
/// Flags affected: N, Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x6e,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&ror_mem)),
};

/// Rotate memory one bit right
///
/// Flags affected: N, Z, C
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x7e,
  cycles: 7,
  extra_cycle: ExtraCycle::None,
  operation: Operation::AbsoluteX(Function::Address(&ror_mem)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::{block::BlockMemory, ReadAddr};

  #[test]
  fn rotate_right_with_carry_clear() {
    let mut core = Core::new(Registers::empty());

    core.reg.status.set(StatusFlags::C_FLAG, false);
    assert_eq!(rotate_right(&mut core, 0b0000_1111), 0b0000_0111);
  }

  #[test]
  fn rotate_right_with_carry_set() {
    let mut core = Core::new(Registers::empty());

    core.reg.status.set(StatusFlags::C_FLAG, true);
    assert_eq!(rotate_right(&mut core, 0b0000_1111), 0b1000_0111);
  }

  #[test]
  fn rotate_right_sets_carry() {
    let mut core = Core::new(Registers::empty());

    rotate_right(&mut core, 0b0000_0010);

    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));

    rotate_right(&mut core, 0b0000_0001);

    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn rotate_right_sets_negative() {
    let mut core = Core::new(Registers::empty());

    rotate_right(&mut core, 0b0000_0000);

    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));

    core.reg.status.set(StatusFlags::C_FLAG, true); // Carry flag has to be set to get a neg result
    rotate_right(&mut core, 0b0000_0000);

    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn rotate_right_sets_zero() {
    let mut core = Core::new(Registers::empty());

    rotate_right(&mut core, 0b0000_0010);

    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));

    rotate_right(&mut core, 0b0000_0000);

    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn ror_acc_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0b0000_0010;
    let operand = core.reg.acc;
    ror_acc(&mut core, operand);
    assert_eq!(core.reg.acc, 0b0000_0001);
  }

  #[test]
  fn ror_mem_impl() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0b0000_0010]);
    let mut core = Core::new(Registers::empty());
    ror_mem(&mut core, &mut memory, 0x00);
    assert_eq!(memory.read_addr(0x00), 0b0000_0001);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("ROR A")[0], ACCUMULATOR.opcode);
    assert_eq!(nes_asm!("ROR $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("ROR $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("ROR $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("ROR $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
