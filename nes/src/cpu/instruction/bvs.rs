use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Brank on overflow set
///
/// Flags affected: none
#[inline(always)]
fn bvs(core: &mut Core, memory: &mut WriteAddr, address: u16) {
  // TODO: implementation
  unimplemented!();
}

/// Brank on overflow set relative
///
/// Flags affected: none
pub const RELATIVE: Instruction = Instruction {
  opcode: 0x70,
  cycles: 2,
  extra_cycle: ExtraCycle::Branch,
  operation: Operation::Relative(Function::Address(&bvs)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;

  #[test]
  fn bvs_impl() {
    let mut core = Core::new(Registers::empty());
    // TODO: test
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BVS $00")[0], RELATIVE.opcode);
  }
}
