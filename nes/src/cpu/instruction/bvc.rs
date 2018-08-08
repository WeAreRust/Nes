use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Brank on overflow clear
///
/// Flags affected: none
#[inline(always)]
fn bvc(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  // TODO: implementation
  unimplemented!();
}

/// Brank on overflow clear relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0x50,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&bvc)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn bvc_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BVC $00")[0], RELATIVE.opcode);
  }
}