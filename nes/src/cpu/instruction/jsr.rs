use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::{Function, Operation},
  Core,
};
use memory::WriteAddr;

/// Jump to new location saving return address
///
/// Flags affected: None
#[inline(always)]
fn jsr(core: &mut Core, memory: &mut WriteAddr, _address: u16) {
  let pc_plus_1: u16 = core.reg.pc + 1;
  let pc_plus_2: u16 = core.reg.pc + 2;

  // push PC + 2
  let pc_plus_2_hi: u8 = (pc_plus_2 >> 8) as u8;
  let pc_plus_2_lo: u8 = pc_plus_2 as u8;
  core.push_stack(memory, pc_plus_2_hi);
  core.push_stack(memory, pc_plus_2_lo);

  // (PC+1) -> PCL
  // (PC+2) -> PCH
  let new_pc_lo = u16::from(memory.read_addr(pc_plus_1));
  let new_pc_hi = u16::from(memory.read_addr(pc_plus_2));
  core.reg.pc = new_pc_lo | (new_pc_hi << 8);
}

/// Jump to new location saving return address
///
/// Flags affected: None
pub const ABSOLUTE: Instruction = Instruction {
  opcode: 0x20,
  cycles: 6,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Absolute(Function::Address(&jsr)),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn jsr_impl() {
    let mut core = Core::new(Registers::empty());
    let mut memory = BlockMemory::with_size(0x0300);
    core.reg.stack = 0xff; // init stack
    core.reg.pc = 0x0200;
    memory.write_addr(0x0201, 0x01); // PC + 1
    memory.write_addr(0x0202, 0xff); // PC + 2
    jsr(&mut core, &mut memory, 0x0200);
    assert_eq!(core.reg.pc, 0xff01);
    assert_eq!(core.pop_stack(&mut memory), 0x02); // PC + 2 (hi)
    assert_eq!(core.pop_stack(&mut memory), 0x02); // PC + 2 (lo)
  }

  #[test]
  fn opcode() {
    assert_eq!(nes_asm!("JSR $0001")[0], ABSOLUTE.opcode);
  }
}
