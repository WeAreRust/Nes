use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Pull accumulator from stack
///
/// Flags affected: N, Z
#[inline(always)]
fn pla(core: &mut Core, memory: &mut WriteAddr) {
  core.reg.acc = core.pop_stack(memory);

  core.reg.status.set_negative(core.reg.acc);
  core.reg.status.set_zero(core.reg.acc);
}

/// Pull accumulator from stack
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x68,
  cycles: 4,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&pla),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::{block::BlockMemory, WriteAddr};

  #[test]
  fn pla_impl() {
    let mut core = Core::new(Registers::empty());
    let mut memory = BlockMemory::with_size(0x01ff);
    memory.write_addr(0x01fe, 0x55);
    core.reg.stack = 0xfe;
    pla(&mut core, &mut memory);
    assert_eq!(core.reg.acc, 0x55);
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("PLA")[0], IMPLIED.opcode);
  }
}
