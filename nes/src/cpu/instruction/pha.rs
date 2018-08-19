use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Push accumulator onto stack
///
/// Flags affected: None
#[inline(always)]
fn pha(core: &mut Core, memory: &mut WriteAddr) {
  core.push_stack(memory, core.reg.acc);
}

/// Push accumulator onto stack
///
/// Flags affected: None
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x48,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&pha),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::{block::BlockMemory, ReadAddr};

  #[test]
  fn pha_impl() {
    let mut core = Core::new(Registers::empty());
    let mut memory = BlockMemory::with_size(0x01ff);
    core.reg.stack = 0xff;
    core.reg.acc = 0x01;
    pha(&mut core, &mut memory);
    assert_eq!(memory.read_addr(0x01fe), 0x01);
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("PHA")[0], IMPLIED.opcode);
  }
}
