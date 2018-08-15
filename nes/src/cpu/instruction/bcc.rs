use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Branch on carry clear
///
/// Flags affected: none
#[inline(always)]
fn bcc(core: &mut Core, _memory: &mut WriteAddr, address: u16) {
  if !core.reg.status.contains(StatusFlags::C_FLAG) {
    core.reg.pc = address;
  }
}

/// Branch on carry clear relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0x90,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&bcc)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn bcc_carry_not_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::C_FLAG, false);
    core.reg.pc = 0x01;

    bcc(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0xFF);
  }

  #[test]
  fn bcc_carry_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::C_FLAG, true);
    core.reg.pc = 0x01;

    bcc(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0x01);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BCC $00")[0], RELATIVE.opcode);
  }
}
