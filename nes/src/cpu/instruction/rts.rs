use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Return from subroutine
///
/// Flags affected: None
#[inline(always)]
fn rts(core: &mut Core, memory: &mut WriteAddr) {
  // pop PC
  let pc_lo = core.pop_stack(memory);
  let pc_hi = core.pop_stack(memory);
  core.reg.pc = (pc_hi as u16) << 8 | pc_lo as u16;

  // INC PC
  core.reg.pc += 1;
}

/// Return from subroutine
///
/// Flags affected: None
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x60,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&rts),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn rts_impl() {
    let mut core = Core::new(Registers::empty());
    let mut memory = BlockMemory::with_size(0x0300);
    core.reg.stack = 0xff; // init stack
    core.reg.pc = 0x0200;
    core.push_stack(&mut memory, 0x03);
    core.push_stack(&mut memory, 0x0e);
    rts(&mut core, &mut memory);
    assert_eq!(core.reg.pc, 0x030f);
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("RTS")[0], IMPLIED.opcode);
  }
}
