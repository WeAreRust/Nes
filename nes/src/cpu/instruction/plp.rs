use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Pull processor status from stack
///
/// Flags affected: All
#[inline(always)]
fn plp(core: &mut Core, memory: &mut WriteAddr) {
  let status_bits = core.pop_stack(memory);
  core.reg.status.adopt(status_bits);
}

/// Pull processor status from stack
///
/// Flags affected: All
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x28,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&plp),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::{block::BlockMemory, WriteAddr};

  #[test]
  fn plp_impl() {
    let mut core = Core::new(Registers::empty());
    let mut memory = BlockMemory::with_size(0x0200);
    memory.write_addr(0x01fe, 0b0101_0101);
    core.reg.stack = 0xfe - 1;
    plp(&mut core, &mut memory);
    let status_bits: u8 = core.reg.status.into();
    assert_eq!(status_bits, 0b0101_0101);
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("PLP")[0], IMPLIED.opcode);
  }
}
