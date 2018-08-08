use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Branch on result zero
///
/// Flags affected: none
#[inline(always)]
fn beq(core: &mut Core, address: u16) {
  // TODO: implementation
  unimplemented!();
}

/// Branch on result zero relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0xF0,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&beq)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn beq_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BEQ $00")[0], RELATIVE.opcode);
  }
}
