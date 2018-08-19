use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Push processor status onto stack
///
/// Flags affected: None
#[inline(always)]
fn php(core: &mut Core, memory: &mut WriteAddr) {
  core.push_stack(memory, core.reg.status.into());
}

/// Push processor status onto stack
///
/// Flags affected: None
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x08,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&php),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::{block::BlockMemory, ReadAddr};

  #[test]
  fn php_impl() {
    let mut core = Core::new(Registers::empty());
    let mut memory = BlockMemory::with_size(0x01ff);
    core.reg.status.set_zero(0x00);
    core.reg.stack = 0xff;
    php(&mut core, &mut memory);
    assert_eq!(memory.read_addr(0x01fe), core.reg.status.into());
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("PHP")[0], IMPLIED.opcode);
  }
}
