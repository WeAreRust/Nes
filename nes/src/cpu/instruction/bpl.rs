use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Branch on result not negative
///
/// Flags affected: none
#[inline(always)]
fn bpl(core: &mut Core, _memory: &mut WriteAddr, address: u16) {
  if !core.reg.status.contains(StatusFlags::N_FLAG) {
    core.reg.pc = address;
  }
}

/// Branch on result not negative relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0x10,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&bpl)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn bpl_negative_not_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::N_FLAG, false);
    core.reg.pc = 0x01;

    bpl(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0xFF);
  }

  #[test]
  fn bpl_negative_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::N_FLAG, true);
    core.reg.pc = 0x01;

    bpl(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0x01);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BPL $00")[0], RELATIVE.opcode);
  }
}
