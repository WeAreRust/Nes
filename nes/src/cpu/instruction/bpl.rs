use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Branch on result not negative
///
/// Flags affected: none
#[inline(always)]
fn bpl(core: &mut Core, address: u16) {
  // TODO: implementation
  unimplemented!();
}

/// Branch on result not negative relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0x10,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&bpl)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn bpl_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BPL $00")[0], RELATIVE.opcode);
  }
}
