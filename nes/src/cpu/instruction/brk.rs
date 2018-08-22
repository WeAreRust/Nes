use cpu::{
  instruction::{ExtraCycle, Instruction},
  operation::Operation,
  register::StatusFlags,
  Core,
};
use memory::WriteAddr;

/// Force break
///
/// Flags affected: I
#[inline(always)]
fn brk(core: &mut Core, memory: &mut WriteAddr) {

  let pc_plus_2 = core.reg.pc + 2;
  let pc_plus_2_hi: u8 = (pc_plus_2 >> 8) as u8;
  let pc_plus_2_lo: u8 = (pc_plus_2 & 0x00FF) as u8;

  // Push PC+2(hi)
  // Push PC+2(lo)
  core.push_stack(memory, pc_plus_2_hi);
  core.push_stack(memory, pc_plus_2_lo);
  // Push "status at the beginning of the break instruction."
  core.push_stack(memory, core.reg.status.into());

  // Set I
  core.reg.status |= StatusFlags::B_FLAG;

  // Fetch PC(lo) from $FFFE
  // Fetch PC(hi) from $FFFF
  let pclo = memory.read_addr(0xFFFE);
  let pchi = memory.read_addr(0xFFFF);

  core.reg.pc = (pchi as u16) << 8 | (pclo as u16);
}

/// Force break
///
/// Flags affected: I
pub const IMPLIED: Instruction = Instruction {
  opcode: 0x00,
  cycles: 7,
  extra_cycle: ExtraCycle::None,
  operation: Operation::Implied(&brk),
};

#[cfg(test)]
mod tests {
  use super::*;
  use cpu::Registers;
  use memory::block::BlockMemory;

  #[test]
  fn brk_impl() {
    let mut core = Core::new(Registers::empty());
    let mut memory = BlockMemory::with_size(0xFFFF + 1);
    memory.write_addr(0xFFFE, 0x10);
    memory.write_addr(0xFFFF, 0x0C);
    core.reg.status |= StatusFlags::N_FLAG;
    core.reg.stack = 0xFF; // init stack
    core.reg.pc = 0x00FE;
    brk(&mut core, &mut memory);

    assert_eq!(core.reg.status, StatusFlags::N_FLAG | StatusFlags::B_FLAG);
    assert_eq!(core.pop_stack(&mut memory), StatusFlags::N_FLAG.into()); // Status flag at start
    assert_eq!(core.pop_stack(&mut memory), 0x00); // PC+2(lo)
    assert_eq!(core.pop_stack(&mut memory), 0x01); // PX+2(hi)
    assert_eq!(core.reg.pc, 0x0C10); // $FFFF -> PCH; $FFFE -> PCL
  }

  #[test]
  fn opcodes() {
    assert_eq!(nes_asm!("BRK")[0], IMPLIED.opcode);
  }
}
