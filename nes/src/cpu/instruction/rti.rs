use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};
use memory::WriteAddr;

/// Return from interrupt
///
/// Flags affected: All
#[inline(always)]
fn rti(core: &mut Core, memory: &mut WriteAddr) {
  core.reg.status = core.pop_stack(memory).into();

  // To pull the PC off the stack, we have to do the reverse of BRK:
  let pc_lo = core.pop_stack(memory);
  let pc_hi = core.pop_stack(memory);
  core.reg.pc = u16::from(pc_hi) << 8 | u16::from(pc_lo);
}

/// Return from interrupt
///
/// Flags affected: All
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x40,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&rti),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::{register::StatusFlags, Registers};
  use memory::block::BlockMemory;

  #[test]
  fn rti_impl() {
    let status = StatusFlags::with_bits(0xff);
    let mut core = Core::new(Registers::empty());
    let mut memory = BlockMemory::with_size(0x0302);
    core.reg.stack = 0xff; // init stack
    core.reg.pc = 0x0200;
    core.push_stack(&mut memory, 0x03);
    core.push_stack(&mut memory, 0x0e);
    core.push_stack(&mut memory, status.into());
    rti(&mut core, &mut memory);
    assert_eq!(core.reg.status, status);
    assert_eq!(core.reg.pc, 0x030e);
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("RTI")[0], IMPLIED.opcode);
  }
}
