use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Rotate operand one bit left
///
/// Flags affected: N, Z, C
#[inline(always)]
fn rotate_left(core: &mut Core, operand: u8) -> u8 {
  let bit_0 = if core.reg.status.contains(StatusFlags::C_FLAG) {
    1
  } else {
    0
  };

  let result = u16::from(operand) << 1 | bit_0;
  let lo = result as u8;

  core.reg.status.set_carry(result);
  core.reg.status.set_negative(lo);
  core.reg.status.set_zero(lo);

  lo
}

/// Rotate accumulator one bit left
///
/// Flags affected: N, Z, C
#[inline(always)]
fn rol_acc(core: &mut Core, _operand: u8) {
  core.reg.acc = rotate_left(core, core.reg.acc);
}

/// Rotate memory one bit left
///
/// Flags affected: N, Z, C
#[inline(always)]
fn rol_mem(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  let result = rotate_left(core, memory.read_addr(address));
  memory.write_addr(address, result);
}

/// Rotate accumulator one bit left
///
/// Flags affected: N, Z, C
pub const ACCUMULATOR: Instruction = Instruction {
  opcode: 0x2a,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Accumulator(&rol_acc),
};

/// Rotate memory one bit left
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x26,
  cycles: 5,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Address(&rol_mem)),
};

/// Rotate memory one bit left
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x36,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Address(&rol_mem)),
};

/// Rotate memory one bit left
///
/// Flags affected: N, Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x2e,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&rol_mem)),
};

/// Rotate memory one bit left
///
/// Flags affected: N, Z, C
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x3e,
  cycles: 7,
  extra_cycle: ExtraCycle::None,
  operation: Operation::AbsoluteX(Function::Address(&rol_mem)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::{block::BlockMemory, ReadAddr};

  #[test]
  fn rotate_left_with_carry_clear() {
    let mut core = Core::new(Registers::empty());

    core.reg.status.set(StatusFlags::C_FLAG, false);
    assert_eq!(rotate_left(&mut core, 0b0000_1111), 0b0001_1110);
  }

  #[test]
  fn rotate_left_with_carry_set() {
    let mut core = Core::new(Registers::empty());

    core.reg.status.set(StatusFlags::C_FLAG, true);
    assert_eq!(rotate_left(&mut core, 0b0000_1111), 0b0001_1111);
  }

  #[test]
  fn rotate_left_sets_carry() {
    let mut core = Core::new(Registers::empty());

    rotate_left(&mut core, 0b0000_0001);

    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));

    rotate_left(&mut core, 0b1000_0001);

    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn rotate_left_sets_negative() {
    let mut core = Core::new(Registers::empty());

    rotate_left(&mut core, 0b0000_0001);

    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));

    rotate_left(&mut core, 0b0100_0000);

    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn rotate_left_sets_zero() {
    let mut core = Core::new(Registers::empty());

    rotate_left(&mut core, 0b0000_0001);

    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));

    rotate_left(&mut core, 0b0000_0000);

    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn rol_acc_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0b0000_0001;
    let operand = core.reg.acc;
    rol_acc(&mut core, operand);
    assert_eq!(core.reg.acc, 0b0000_0010);
  }

  #[test]
  fn rol_mem_impl() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0b0000_0001]);
    let mut core = Core::new(Registers::empty());
    rol_mem(&mut core, &mut memory, 0x00);
    assert_eq!(memory.read_addr(0x00), 0b0000_0010);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("ROL A")[0], ACCUMULATOR.opcode);
    assert_eq!(nes_asm!("ROL $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("ROL $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("ROL $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("ROL $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
