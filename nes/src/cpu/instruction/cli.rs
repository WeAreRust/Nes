use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Clear interrupt disable bit
///
/// Flags affected: I
#[inline(always)]
fn cli(core: &mut Core, _memory: &mut WriteAddr) {
  core.reg.status.set(StatusFlags::I_FLAG, false)
}

/// Clear interrupt disable bit
///
/// Flags affected: I
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x58,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&cli),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn cli_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.status.set(StatusFlags::I_FLAG, true);
    cli(&mut core, &mut BlockMemory::with_size(0));
    assert!(!core.reg.status.contains(StatusFlags::I_FLAG));
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("CLI")[0], IMPLIED.opcode);
  }
}
