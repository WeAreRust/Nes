use cpu::{
  instruction::Instruction,
  operation::{Function, Operation},
  Core,
};

/// Brank on overflow clear
///
/// Flags affected: none
#[inline(always)]
fn bvc(core: &mut Core, address: u16) {
  // TODO: implementation
  unimplemented!();
}

/// Brank on overflow clear relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0x50,
  cycles: 2,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: true,
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
