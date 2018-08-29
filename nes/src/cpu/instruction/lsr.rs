use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Shift operand one bit right
///
/// Flags affected: Z, C
#[inline(always)]
fn shift_right(core: &mut Core, operand: u8) -> u8 {
  let value = operand >> 1;

  // Move the 1st operand but into the 9th u16 bit for the carry test
  core.reg.status.set_carry(u16::from(operand) << 8);
  core.reg.status.set_zero(value);

  value
}

/// Shift accumulator one bit right
///
/// Flags affected: Z, C
#[inline(always)]
fn lsr_acc(core: &mut Core, operand: u8) {
  core.reg.acc = shift_right(core, operand);
}

/// Shift memory one bit right
///
/// Flags affected: Z, C
#[inline(always)]
fn lsr_mem(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  let value = shift_right(core, memory.read_addr(address));
  memory.write_addr(address, value);
}

/// Shift accumulator one bit right
///
/// Flags affected: Z, C
pub const ACCUMULATOR: Instruction = Instruction {
  opcode: 0x4a,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Accumulator(&lsr_acc),
};

/// Shift memory one bit right
///
/// Flags affected: Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x46,
  cycles: 5,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Address(&lsr_mem)),
};

/// Shift memory one bit right
///
/// Flags affected: Z, C
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x56,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Address(&lsr_mem)),
};

/// Shift memory one bit right
///
/// Flags affected: Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x4e,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&lsr_mem)),
};

/// Shift memory one bit right
///
/// Flags affected: Z, C
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x5e,
  cycles: 7,
  extra_cycle: ExtraCycle::None,
  operation: Operation::AbsoluteX(Function::Address(&lsr_mem)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};
  use memory::{block::BlockMemory, ReadAddr};

  #[test]
  fn shift_right_impl() {
    let mut core = Core::new(Registers::empty());
    assert_eq!(shift_right(&mut core, 0b0000_0010), 0b0000_0001);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn shift_right_impl_zero() {
    let mut core = Core::new(Registers::empty());
    assert_eq!(shift_right(&mut core, 0b0000_0000), 0b0000_0000);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn shift_right_impl_carry() {
    let mut core = Core::new(Registers::empty());
    assert_eq!(shift_right(&mut core, 0b0000_0011), 0b0000_0001);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn lsr_acc_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0b0000_0010;
    let operand = core.reg.acc;
    lsr_acc(&mut core, operand);
    assert_eq!(core.reg.acc, 0b0000_0001);
  }

  #[test]
  fn lsr_mem_impl() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0b0000_0010]);
    let mut core = Core::new(Registers::empty());
    lsr_mem(&mut core, &mut memory, 0x00);
    assert_eq!(memory.read_addr(0x00), 0b0000_0001);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("LSR A")[0], ACCUMULATOR.opcode);
    assert_eq!(nes_asm!("LSR $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("LSR $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("LSR $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("LSR $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
