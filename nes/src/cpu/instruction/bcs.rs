use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Branch on carry set
///
/// Flags affected: none
#[inline(always)]
fn bcs(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  // TODO: implementation
  unimplemented!();
}

/// Branch on carry set relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0xB0,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&bcs)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn bcs_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BCS $00")[0], RELATIVE.opcode);
  }
}
