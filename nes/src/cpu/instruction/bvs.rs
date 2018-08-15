use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Branch on overflow set
///
/// Flags affected: none
#[inline(always)]
fn bvs(core: &mut Core, _memory: &mut WriteAddr, address: u16) {
  if core.reg.status.contains(StatusFlags::V_FLAG) {
    core.reg.pc = address;
  }
}

/// Branch on overflow set relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0x70,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&bvs)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn bvs_overflow_not_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::V_FLAG, false);
    core.reg.pc = 0x01;

    bvs(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0x01);
  }

  #[test]
  fn bvs_overflow_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::V_FLAG, true);
    core.reg.pc = 0x01;

    bvs(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0xFF);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BVS $00")[0], RELATIVE.opcode);
  }
}
