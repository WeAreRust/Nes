use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

use memory::WriteAddr;

/// Push processor status onto stack
///
/// Flags affected: None
#[inline(always)]
fn php(_core: &mut Core, _memory: &mut WriteAddr) {
  // TODO: implementation
}

/// Push processor status onto stack
///
/// Flags affected: None
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x08,
  cycles: 3,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&php),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn php_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("PHP")[0], IMPLIED.opcode);
  }
}
