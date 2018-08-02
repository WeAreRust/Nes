use cpu::{
  instruction::Instruction,
  operation::{Function, Operation},
  Core,
};

/// Branch on carry clear
///
/// Flags affected: none
#[inline(always)]
fn bcc(core: &mut Core, address: u16) {
  // TODO: implementation
  unimplemented!();
}

/// Branch on carry clear relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0x90,
  cycles: 2,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: true,
  operation: Operation::Relative(Function::Address(&bcc)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn bcc_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BCC $00")[0], RELATIVE.opcode);
  }
}
