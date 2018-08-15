use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Branch on carry set
///
/// Flags affected: none
#[inline(always)]
fn bcs(core: &mut Core, _memory: &mut WriteAddr, address: u16) {
  if core.reg.status.contains(StatusFlags::C_FLAG) {
    core.reg.pc = address;
  }
}

/// Branch on carry set relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0xB0,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&bcs)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn bcs_carry_not_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::C_FLAG, false);
    core.reg.pc = 0x01;

    bcs(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0x01);
  }

  #[test]
  fn bcs_carry_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::C_FLAG, true);
    core.reg.pc = 0x01;

    bcs(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0xFF);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BCS $00")[0], RELATIVE.opcode);
  }
}
