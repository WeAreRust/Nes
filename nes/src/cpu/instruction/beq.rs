use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Branch on result zero
///
/// Flags affected: none
#[inline(always)]
fn beq(core: &mut Core, _memory: &mut WriteAddr, address: u16) {
  if core.reg.status.contains(StatusFlags::Z_FLAG) {
    core.reg.pc = address;
  }
}

/// Branch on result zero relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0xF0,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&beq)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn beq_zero_not_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::Z_FLAG, false);
    core.reg.pc = 0x01;

    beq(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0x01);
  }

  #[test]
  fn beq_zero_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::Z_FLAG, true);
    core.reg.pc = 0x01;

    beq(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0xFF);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BEQ $00")[0], RELATIVE.opcode);
  }
}
