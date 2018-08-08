use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};

/// Branch on result minus
///
/// Flags affected: none
#[inline(always)]
fn bmi(core: &mut Core, address: u16) {
  // TODO: implementation
  unimplemented!();
}

/// Branch on result minus relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0x30,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&bmi)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn bmi_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BMI $00")[0], RELATIVE.opcode);
  }
}
