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

mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod brk;
mod bvc;
mod bvs;
mod clc;
mod cld;
mod cli;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod dex;
mod dey;
mod jmp;
mod lda;
mod nop;

// TODO(benjaminjt): Generate this with a macro, e.g. instruction_set![and::immediate, ...]
mod instruction_set {
  use super::*;

  pub fn get(opcode: u8) -> Instruction {
    match opcode {
      o if o == adc::IMMEDIATE.opcode => adc::IMMEDIATE,
      o if o == adc::ZERO_PAGE.opcode => adc::ZERO_PAGE,
      o if o == adc::ZERO_PAGE_X.opcode => adc::ZERO_PAGE_X,
      o if o == adc::ABSOLUTE.opcode => adc::ABSOLUTE,
      o if o == adc::ABSOLUTE_X.opcode => adc::ABSOLUTE_X,
      o if o == adc::ABSOLUTE_Y.opcode => adc::ABSOLUTE_Y,
      o if o == adc::INDIRECT_X.opcode => adc::INDIRECT_X,
      o if o == adc::INDIRECT_Y.opcode => adc::INDIRECT_Y,
      o if o == and::IMMEDIATE.opcode => and::IMMEDIATE,
      o if o == and::ZERO_PAGE.opcode => and::ZERO_PAGE,
      o if o == and::ZERO_PAGE_X.opcode => and::ZERO_PAGE_X,
      o if o == and::ABSOLUTE.opcode => and::ABSOLUTE,
      o if o == and::ABSOLUTE_X.opcode => and::ABSOLUTE_X,
      o if o == and::ABSOLUTE_Y.opcode => and::ABSOLUTE_Y,
      o if o == and::INDIRECT_X.opcode => and::INDIRECT_X,
      o if o == and::INDIRECT_Y.opcode => and::INDIRECT_Y,
      o if o == asl::ACCUMULATOR.opcode => asl::ACCUMULATOR,
      o if o == asl::ZERO_PAGE.opcode => asl::ZERO_PAGE,
      o if o == asl::ZERO_PAGE_X.opcode => asl::ZERO_PAGE_X,
      o if o == asl::ABSOLUTE.opcode => asl::ABSOLUTE,
      o if o == asl::ABSOLUTE_X.opcode => asl::ABSOLUTE_X,
      o if o == bcc::RELATIVE.opcode => bcc::RELATIVE,
      o if o == bcs::RELATIVE.opcode => bcs::RELATIVE,
      o if o == beq::RELATIVE.opcode => beq::RELATIVE,
      o if o == bit::ZERO_PAGE.opcode => bit::ZERO_PAGE,
      o if o == bit::ABSOLUTE.opcode => bit::ABSOLUTE,
      o if o == bmi::RELATIVE.opcode => bmi::RELATIVE,
      o if o == bne::RELATIVE.opcode => bne::RELATIVE,
      o if o == bpl::RELATIVE.opcode => bpl::RELATIVE,
      o if o == brk::IMPLIED.opcode => brk::IMPLIED,
      o if o == bvc::RELATIVE.opcode => bvc::RELATIVE,
      o if o == bvs::RELATIVE.opcode => bvs::RELATIVE,
      o if o == clc::IMPLIED.opcode => clc::IMPLIED,
      o if o == cld::IMPLIED.opcode => cld::IMPLIED,
      o if o == cli::IMPLIED.opcode => cli::IMPLIED,
      o if o == clv::IMPLIED.opcode => clv::IMPLIED,
      o if o == cmp::IMMEDIATE.opcode => cmp::IMMEDIATE,
      o if o == cmp::ZERO_PAGE.opcode => cmp::ZERO_PAGE,
      o if o == cmp::ZERO_PAGE_X.opcode => cmp::ZERO_PAGE_X,
      o if o == cmp::ABSOLUTE.opcode => cmp::ABSOLUTE,
      o if o == cmp::ABSOLUTE_X.opcode => cmp::ABSOLUTE_X,
      o if o == cmp::ABSOLUTE_Y.opcode => cmp::ABSOLUTE_Y,
      o if o == cmp::INDIRECT_X.opcode => cmp::INDIRECT_X,
      o if o == cmp::INDIRECT_Y.opcode => cmp::INDIRECT_Y,
      o if o == cpx::IMMEDIATE.opcode => cpx::IMMEDIATE,
      o if o == cpx::ZERO_PAGE.opcode => cpx::ZERO_PAGE,
      o if o == cpx::ABSOLUTE.opcode => cpx::ABSOLUTE,
      o if o == cpy::IMMEDIATE.opcode => cpy::IMMEDIATE,
      o if o == cpy::ZERO_PAGE.opcode => cpy::ZERO_PAGE,
      o if o == cpy::ABSOLUTE.opcode => cpy::ABSOLUTE,
      o if o == dec::ZERO_PAGE.opcode => dec::ZERO_PAGE,
      o if o == dec::ZERO_PAGE_X.opcode => dec::ZERO_PAGE_X,
      o if o == dec::ABSOLUTE.opcode => dec::ABSOLUTE,
      o if o == dec::ABSOLUTE_X.opcode => dec::ABSOLUTE_X,
      o if o == dex::IMPLIED.opcode => dex::IMPLIED,
      o if o == dey::IMPLIED.opcode => dey::IMPLIED,
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

  /// add 1 to cycles if page boundery is crossed
  page_boundary_extra_cycle: bool,

  /// add 1 to cycles if branch occurs on same page
  /// add 2 to cycles if branch occurs to different page
  page_branch_extra_cycles: bool,

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
  pub fn cycles(&self, core: &Core, memory: &mut ReadAddr) -> usize {
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
