use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Branch on result not zero
///
/// Flags affected: none
#[inline(always)]
fn bne(core: &mut Core, _memory: &mut WriteAddr, address: u16) {
  if !core.reg.status.contains(StatusFlags::Z_FLAG) {
    core.reg.pc = address;
  }
}

/// Branch on result not zero relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0xd0,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&bne)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn bne_zero_not_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::Z_FLAG, false);
    core.reg.pc = 0x01;

    bne(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0xFF);
  }

  #[test]
  fn bne_zero_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::Z_FLAG, true);
    core.reg.pc = 0x01;

    bne(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0x01);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BNE $00")[0], RELATIVE.opcode);
  }
}
