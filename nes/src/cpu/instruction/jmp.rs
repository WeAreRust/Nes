use cpu::{
  instruction::Instruction,
  operation::{Function, Operation},
  Core,
};

/// Jump to address
///
/// Flags affected: None
#[inline(always)]
fn jump(core: &mut Core, address: u16) {
  core.reg.pc = address;
}

/// Jump absolute
///
/// Flags affected: None
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x4c,
  cycles: 3,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::Absolute(Function::Address(&jump)),
};

/// Jump indirect
///
/// Flags affected: None
///
/// An indirect jump must never use a vector beginning on the last byte of a page. If this
/// occurs then the low byte should be as expected, and the high byte should wrap to the start
/// of the page. See http://www.6502.org/tutorials/6502opcodes.html#JMP for details.
pub const INDIRECT: Instruction = Instruction {
  opcode: 0x6c,
  cycles: 5,
  page_boundary_extra_cycle: false,
  page_branch_extra_cycles: false,
  operation: Operation::Indirect(Function::Address(&jump)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn jump_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.pc = 0x0001;
    jump(&mut core, 0x000F);
    assert_eq!(core.reg.pc, 0x000F);
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("JMP $0001")[0], ABSOLUTE.opcode);
    assert_eq!(nes_asm!("JMP ($0001)")[0], INDIRECT.opcode);
  }
}
