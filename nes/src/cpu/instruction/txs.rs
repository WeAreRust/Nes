use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  Core,
};

/// Transfer index x to stack register
///
/// Flags affected: None
#[inline(always)]
fn txs(core: &mut Core) {
  core.reg.stack = core.reg.x_idx;
}

/// Transfer index x to stack register
///
/// Flags affected: None
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x9a,
  cycles: 2,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&txs),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn txs_impl() {
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 1;
    txs(&mut core);
    assert_eq!(core.reg.stack, 1);
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("TXS")[0], IMPLIED.opcode);
  }
}
