use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Transfer index y to accumulator
///
/// Flags affected: N, Z
#[inline(always)]
fn tya(core: &mut Core) {
  // TODO: implementation
}

/// Transfer index y to accumulator
///
/// Flags affected: N, Z
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x98,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&tya),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn tya_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("TYA")[0], IMPLIED.opcode);
  }
}
