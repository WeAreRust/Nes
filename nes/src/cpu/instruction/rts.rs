use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Return from subroutine
///
/// Flags affected: None
#[inline(always)]
fn rts(core: &mut Core) {
  // TODO: implementation
  unimplemented!();
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

  #[test]
  fn rti_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("RTS")[0], IMPLIED.opcode);
  }
}
