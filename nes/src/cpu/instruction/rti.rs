use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Return from interrupt
///
/// Flags affected: All
#[inline(always)]
fn rti(core: &mut Core) {
  // TODO: implementation
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
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("RTI")[0], IMPLIED.opcode);
  }
}