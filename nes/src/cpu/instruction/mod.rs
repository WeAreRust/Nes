use cpu::operation::Operation;
use cpu::Core;
use memory::{ReadAddr, WriteAddr};
use std::convert::From;

#[cfg(test)]
#[macro_export]
macro_rules! nes_asm {
  ($e:expr) => {{
    let mut buf = vec![];
    // We push a newline into the bytes array because of a known issue in asm6502
    $crate::asm6502::assemble(format!("{}\n", $e).as_bytes(), &mut buf).unwrap();
    buf
  }};
}

mod def;

mod and;
mod jmp;
mod lda;
mod nop;

// TODO(benjaminjt): Generate this with a macro, e.g. instruction_set![and::immediate, ...]
mod instruction_set {
  use super::*;

  pub fn get(opcode: u8) -> Instruction {
    match opcode {
      o if o == and::IMMEDIATE.opcode => and::IMMEDIATE,
      o if o == and::ZERO_PAGE.opcode => and::ZERO_PAGE,
      o if o == and::ZERO_PAGE_X.opcode => and::ZERO_PAGE_X,
      o if o == and::ABSOLUTE.opcode => and::ABSOLUTE,
      o if o == and::ABSOLUTE_X.opcode => and::ABSOLUTE_X,
      o if o == and::ABSOLUTE_Y.opcode => and::ABSOLUTE_Y,
      o if o == and::INDIRECT_X.opcode => and::INDIRECT_X,
      o if o == and::INDIRECT_Y.opcode => and::INDIRECT_Y,
      o if o == jmp::ABSOLUTE.opcode => jmp::ABSOLUTE,
      o if o == jmp::INDIRECT.opcode => jmp::INDIRECT,
      o if o == lda::IMMEDIATE.opcode => lda::IMMEDIATE,
      o if o == lda::ZERO_PAGE.opcode => lda::ZERO_PAGE,
      o if o == lda::ZERO_PAGE_X.opcode => lda::ZERO_PAGE_X,
      o if o == lda::ABSOLUTE.opcode => lda::ABSOLUTE,
      o if o == lda::ABSOLUTE_X.opcode => lda::ABSOLUTE_X,
      o if o == lda::ABSOLUTE_Y.opcode => lda::ABSOLUTE_Y,
      o if o == lda::INDIRECT_X.opcode => lda::INDIRECT_X,
      o if o == lda::INDIRECT_Y.opcode => lda::INDIRECT_Y,
      o if o == nop::IMPLIED.opcode => nop::IMPLIED,
      _ => panic!("instruction not implemented: 0x{:02X}", opcode),
    }
  }
}

pub struct Instruction {
  opcode: u8,
  cycles: usize,
  page_boundary_extra_cycle: bool,
  operation: Operation,
}

impl From<u8> for Instruction {
  fn from(opcode: u8) -> Self {
    instruction_set::get(opcode)
  }
}

impl Instruction {
  #[inline(always)]
  pub fn opcode(&self) -> u8 {
    self.opcode
  }

  #[inline(always)]
  pub fn base_cycles(&self) -> usize {
    self.cycles
  }

  // TODO: test.
  pub fn cycles(&self, core: &Core, memory: &ReadAddr) -> usize {
    if !self.page_boundary_extra_cycle {
      return self.cycles;
    }

    let lo = u16::from(memory.read_addr(core.reg.pc));
    let hi = u16::from(memory.read_addr(core.reg.pc + 1));
    let addr = lo | hi << 8;

    self.cycles + is_upper_page_boundary(addr) as usize
  }

  // TODO: test.
  pub fn execute<M: ReadAddr + WriteAddr>(&self, core: &mut Core, memory: &mut M) {
    core.reg.pc += 1;
    self.operation.call(core, memory);
  }
}

#[inline(always)]
pub fn is_upper_page_boundary(addr: u16) -> bool {
  addr & 0x00ff == 0x00ff
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn page_boundary() {
    assert!(is_upper_page_boundary(0x30ff));

    assert!(!is_upper_page_boundary(0x30fe));
    assert!(!is_upper_page_boundary(0x3100));
  }
}
