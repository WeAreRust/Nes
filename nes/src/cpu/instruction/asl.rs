use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Shift operand left one bit, returning the lo 8 bits as u8
///
/// Flags affected: N, Z, C
fn shift_left(core: &mut Core, operand: u8) -> u8 {
  let value: u16 = (operand as u16) << 1;
  let lo_value = value as u8;

  core.reg.status.set_carry(value);
  core.reg.status.set_zero(lo_value);
  core.reg.status.set_negative(lo_value);

  lo_value
}

/// Shift accumulator left one bit
///
/// Flags affected: N, Z, C
#[inline(always)]
fn asl_acc(core: &mut Core, _operand: u8) {
  core.reg.acc = shift_left(core, core.reg.acc); // Place the lo 8 bits into acc.
}

/// Shift memory left one bit
///
/// Flags affected: N, Z, C
#[inline(always)]
fn asl_mem(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  let lo_value = shift_left(core, memory.read_addr(address));
  memory.write_addr(address, lo_value); // Place the lo 8 bits back into memory.
}

/// Shift accumulator left one bit
///
/// Flags affected: N, Z, C
pub const ACCUMULATOR: Instruction = Instruction {
  opcode: 0x0a,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Accumulator(&asl_acc),
};

/// Shift memory left one bit zero page
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x06,
  cycles: 5,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Address(&asl_mem)),
};

/// Shift memory left one bit zero page X
///
/// Flags affected: N, Z, C
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x16,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Address(&asl_mem)),
};

/// Shift memory left one bit absolute
///
/// Flags affected: N, Z, C
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x0e,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&asl_mem)),
};

/// Shift memory left one bit absolute X
///
/// Flags affected: N, Z, C
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0x1e,
  cycles: 7,
  extra_cycle: ExtraCycle::None,
  operation: Operation::AbsoluteX(Function::Address(&asl_mem)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};
  use memory::{block::BlockMemory, ReadAddr};

  #[test]
  fn shift_left_impl() {
    let mut core = Core::new(Registers::empty());
    assert_eq!(shift_left(&mut core, 0b_0000_0001), 0b_0000_0010);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn shift_left_impl_negative() {
    let mut core = Core::new(Registers::empty());
    assert_eq!(shift_left(&mut core, 0b_0100_0000), 0b_1000_0000);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn shift_left_impl_zero() {
    let mut core = Core::new(Registers::empty());
    assert_eq!(shift_left(&mut core, 0b_0000_0000), 0b_0000_0000);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn shift_left_impl_carry() {
    let mut core = Core::new(Registers::empty());
    assert_eq!(shift_left(&mut core, 0b_1000_0001), 0b_0000_0010);
    assert!(!core.reg.status.contains(StatusFlags::N_FLAG));
    assert!(!core.reg.status.contains(StatusFlags::Z_FLAG));
    assert!(core.reg.status.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn asl_acc_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.acc = 0b_0000_0001;
    let operand = core.reg.acc;
    asl_acc(&mut core, operand);
    assert_eq!(core.reg.acc, 0b_0000_0010);
  }

  #[test]
  fn asl_mem_impl() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0b_0000_0001]);
    let mut core = Core::new(Registers::empty());
    asl_mem(&mut core, &mut memory, 0x00);
    assert_eq!(memory.read_addr(0x00), 0b_0000_0010);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("ASL A")[0], ACCUMULATOR.opcode);
    assert_eq!(nes_asm!("ASL $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("ASL $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("ASL $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("ASL $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
