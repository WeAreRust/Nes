use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Store index y in memory
///
/// Flags affected: None
#[inline(always)]
fn sty(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  memory.write_addr(address, core.reg.y_idx);
}

/// Store index y in memory
///
/// Flags affected: None
pub const ZERO_PAGE: Instruction = Instruction {
  opcode: 0x84,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPage(Function::Address(&sty)),
};

/// Store index y in memory
///
/// Flags affected: None
pub const ZERO_PAGE_X: Instruction = Instruction {
  opcode: 0x94,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::ZeroPageX(Function::Address(&sty)),
};

/// Store index y in memory
///
/// Flags affected: None
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x8c,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&sty)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::{block::BlockMemory, ReadAddr};

  #[test]
  fn sty_impl() {
    let mut memory = BlockMemory::with_bytes(vec![0xff]);
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 1;
    sty(&mut core, &mut memory, 0);
    assert_eq!(memory.read_addr(0), 1);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("STY $00")[0], ZERO_PAGE.opcode);
    assert_eq!(nes_asm!("STY $00,X")[0], ZERO_PAGE_X.opcode);
    assert_eq!(nes_asm!("STY $0000")[0], ABSOLUTE.opcode);
  }
}
