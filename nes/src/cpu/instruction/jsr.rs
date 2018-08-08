use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Jump to new location saving return address
///
/// Flags affected: None
#[inline(always)]
fn jsr(core: &mut Core, address: u16) {
  // TODO: implementation
}

/// Jump to new location saving return address
///
/// Flags affected: None
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x20,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&jsr)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn jsr_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("JSR $0001")[0], ABSOLUTE.opcode);
  }
}
