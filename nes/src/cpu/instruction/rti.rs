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
  use cpu::Registers;

  #[test]
  fn rti_impl() {
    let mut _core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("RTI")[0], IMPLIED.opcode);
  }
}
