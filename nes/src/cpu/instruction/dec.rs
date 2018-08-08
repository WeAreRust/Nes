use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Decrement memory by one
///
/// Flags affected: N, Z
#[inline(always)]
fn dec(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  let value = u8::wrapping_sub(memory.read_addr(address), 1);
  memory.write_addr(address, value);

  core.reg.status.set_zero(value);
  core.reg.status.set_negative(value);
}

/// Decrement memory by one zero page
///
/// Flags affected: N, Z
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0xc6,
  cycles: 5,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Address(&dec)),
};

/// Decrement memory by one zero page X
///
/// Flags affected: N, Z
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0xd6,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Address(&dec)),
};

/// Decrement memory by one absolute
///
/// Flags affected: N, Z
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0xce,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&dec)),
};

/// Decrement memory by one absolute X
///
/// Flags affected: N, Z
pub const ABSOLUTE_X: Instruction = Instruction {
  opcode: 0xde,
  cycles: 7,
  extra_cycle: ExtraCycle::Boundary,
  operation: Operation::AbsoluteX(Function::Address(&dec)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};
  use memory::{block::BlockMemory, ReadAddr};

  #[test]
  fn dec_impl() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00, 0xff]);
    let mut core = Core::new(Registers::empty());
    dec(&mut core, &mut memory, 0x01);
    assert_eq!(memory.read_addr(0x01), 0xfe);
  }

  #[test]
  fn dec_impl_overflow() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00, 0x00]);
    let mut core = Core::new(Registers::empty());
    dec(&mut core, &mut memory, 0x01);
    assert_eq!(memory.read_addr(0x01), 0xff);
  }

  #[test]
  fn dec_impl_zero_flag() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x01]);
    let mut core = Core::new(Registers::empty());
    dec(&mut core, &mut memory, 0x00);
    assert_eq!(memory.read_addr(0x00), 0);
    assert!(core.reg.status.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn dec_impl_negative_flag() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![129]);
    let mut core = Core::new(Registers::empty());
    dec(&mut core, &mut memory, 0x00);
    assert_eq!(memory.read_addr(0x00), 128);
    assert!(core.reg.status.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("DEC $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("DEC $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("DEC $0000")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("DEC $0000,X")[0], ABSOLUTE_X.opcode);
  }
}
