use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Branch on result minus
///
/// Flags affected: none
#[inline(always)]
fn bmi(core: &mut Core, _memory: &mut WriteAddr, address: u16) {
  if core.reg.status.contains(StatusFlags::N_FLAG) {
    core.reg.pc = address;
  }
}

/// Branch on result minus relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0x30,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&bmi)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn bmi_negative_not_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::N_FLAG, false);
    core.reg.pc = 0x01;

    bmi(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0x01);
  }

  #[test]
  fn bmi_negative_set() {
    let mut memory: BlockMemory = BlockMemory::with_bytes(vec![0x00]);
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::N_FLAG, true);
    core.reg.pc = 0x01;

    bmi(&mut core, &mut memory, 0xFF);

    assert_eq!(core.reg.pc, 0xFF);
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BMI $00")[0], RELATIVE.opcode);
  }
}
