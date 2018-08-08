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

#[cfg(test)]
pub fn into_byte(num: i8) -> u8 {
  num as u8
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
mod eor;
mod inc;
mod inx;
mod iny;
mod jmp;
mod jsr;
mod lda;
mod ldx;
mod ldy;
mod lsr;
mod nop;
mod ora;
mod pha;
mod php;
mod pla;
mod plp;
mod rol;
mod ror;
mod rti;
mod rts;
mod sbc;
mod sec;
mod sed;
mod sei;
mod sta;
mod stx;
mod sty;
mod tax;
mod tay;
mod tsx;
mod txa;
mod txs;
mod tya;

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
      o if o == eor::IMMEDIATE.opcode => eor::IMMEDIATE,
      o if o == eor::ZERO_PAGE.opcode => eor::ZERO_PAGE,
      o if o == eor::ZERO_PAGE_X.opcode => eor::ZERO_PAGE_X,
      o if o == eor::ABSOLUTE.opcode => eor::ABSOLUTE,
      o if o == eor::ABSOLUTE_X.opcode => eor::ABSOLUTE_X,
      o if o == eor::ABSOLUTE_Y.opcode => eor::ABSOLUTE_Y,
      o if o == eor::INDIRECT_X.opcode => eor::INDIRECT_X,
      o if o == eor::INDIRECT_Y.opcode => eor::INDIRECT_Y,
      o if o == inc::ZERO_PAGE.opcode => inc::ZERO_PAGE,
      o if o == inc::ZERO_PAGE_X.opcode => inc::ZERO_PAGE_X,
      o if o == inc::ABSOLUTE.opcode => inc::ABSOLUTE,
      o if o == inc::ABSOLUTE_X.opcode => inc::ABSOLUTE_X,
      o if o == inx::IMPLIED.opcode => inx::IMPLIED,
      o if o == iny::IMPLIED.opcode => iny::IMPLIED,
      o if o == jmp::ABSOLUTE.opcode => jmp::ABSOLUTE,
      o if o == jmp::INDIRECT.opcode => jmp::INDIRECT,
      o if o == jsr::ABSOLUTE.opcode => jsr::ABSOLUTE,
      o if o == lda::IMMEDIATE.opcode => lda::IMMEDIATE,
      o if o == lda::ZERO_PAGE.opcode => lda::ZERO_PAGE,
      o if o == lda::ZERO_PAGE_X.opcode => lda::ZERO_PAGE_X,
      o if o == lda::ABSOLUTE.opcode => lda::ABSOLUTE,
      o if o == lda::ABSOLUTE_X.opcode => lda::ABSOLUTE_X,
      o if o == lda::ABSOLUTE_Y.opcode => lda::ABSOLUTE_Y,
      o if o == lda::INDIRECT_X.opcode => lda::INDIRECT_X,
      o if o == lda::INDIRECT_Y.opcode => lda::INDIRECT_Y,
      o if o == ldx::IMMEDIATE.opcode => ldx::IMMEDIATE,
      o if o == ldx::ZERO_PAGE.opcode => ldx::ZERO_PAGE,
      o if o == ldx::ZERO_PAGE_Y.opcode => ldx::ZERO_PAGE_Y,
      o if o == ldx::ABSOLUTE.opcode => ldx::ABSOLUTE,
      o if o == ldx::ABSOLUTE_Y.opcode => ldx::ABSOLUTE_Y,
      o if o == ldy::IMMEDIATE.opcode => ldy::IMMEDIATE,
      o if o == ldy::ZERO_PAGE.opcode => ldy::ZERO_PAGE,
      o if o == ldy::ZERO_PAGE_X.opcode => ldy::ZERO_PAGE_X,
      o if o == ldy::ABSOLUTE.opcode => ldy::ABSOLUTE,
      o if o == ldy::ABSOLUTE_X.opcode => ldy::ABSOLUTE_X,
      o if o == lsr::ACCUMULATOR.opcode => lsr::ACCUMULATOR,
      o if o == lsr::ZERO_PAGE.opcode => lsr::ZERO_PAGE,
      o if o == lsr::ZERO_PAGE_X.opcode => lsr::ZERO_PAGE_X,
      o if o == lsr::ABSOLUTE.opcode => lsr::ABSOLUTE,
      o if o == lsr::ABSOLUTE_X.opcode => lsr::ABSOLUTE_X,
      o if o == nop::IMPLIED.opcode => nop::IMPLIED,
      o if o == ora::IMMEDIATE.opcode => ora::IMMEDIATE,
      o if o == ora::ZERO_PAGE.opcode => ora::ZERO_PAGE,
      o if o == ora::ZERO_PAGE_X.opcode => ora::ZERO_PAGE_X,
      o if o == ora::ABSOLUTE.opcode => ora::ABSOLUTE,
      o if o == ora::ABSOLUTE_X.opcode => ora::ABSOLUTE_X,
      o if o == ora::ABSOLUTE_Y.opcode => ora::ABSOLUTE_Y,
      o if o == ora::INDIRECT_X.opcode => ora::INDIRECT_X,
      o if o == ora::INDIRECT_Y.opcode => ora::INDIRECT_Y,
      o if o == pha::IMPLIED.opcode => pha::IMPLIED,
      o if o == php::IMPLIED.opcode => php::IMPLIED,
      o if o == pla::IMPLIED.opcode => pla::IMPLIED,
      o if o == plp::IMPLIED.opcode => plp::IMPLIED,
      o if o == rol::ACCUMULATOR.opcode => rol::ACCUMULATOR,
      o if o == rol::ZERO_PAGE.opcode => rol::ZERO_PAGE,
      o if o == rol::ZERO_PAGE_X.opcode => rol::ZERO_PAGE_X,
      o if o == rol::ABSOLUTE.opcode => rol::ABSOLUTE,
      o if o == rol::ABSOLUTE_X.opcode => rol::ABSOLUTE_X,
      o if o == ror::ACCUMULATOR.opcode => ror::ACCUMULATOR,
      o if o == ror::ZERO_PAGE.opcode => ror::ZERO_PAGE,
      o if o == ror::ZERO_PAGE_X.opcode => ror::ZERO_PAGE_X,
      o if o == ror::ABSOLUTE.opcode => ror::ABSOLUTE,
      o if o == ror::ABSOLUTE_X.opcode => ror::ABSOLUTE_X,
      o if o == rti::IMPLIED.opcode => rti::IMPLIED,
      o if o == rts::IMPLIED.opcode => rts::IMPLIED,
      o if o == sbc::IMMEDIATE.opcode => sbc::IMMEDIATE,
      o if o == sbc::ZERO_PAGE.opcode => sbc::ZERO_PAGE,
      o if o == sbc::ZERO_PAGE_X.opcode => sbc::ZERO_PAGE_X,
      o if o == sbc::ABSOLUTE.opcode => sbc::ABSOLUTE,
      o if o == sbc::ABSOLUTE_X.opcode => sbc::ABSOLUTE_X,
      o if o == sbc::ABSOLUTE_Y.opcode => sbc::ABSOLUTE_Y,
      o if o == sbc::INDIRECT_X.opcode => sbc::INDIRECT_X,
      o if o == sbc::INDIRECT_Y.opcode => sbc::INDIRECT_Y,
      o if o == sec::IMPLIED.opcode => sec::IMPLIED,
      o if o == sed::IMPLIED.opcode => sed::IMPLIED,
      o if o == sei::IMPLIED.opcode => sei::IMPLIED,
      o if o == sta::ZERO_PAGE.opcode => sta::ZERO_PAGE,
      o if o == sta::ZERO_PAGE_X.opcode => sta::ZERO_PAGE_X,
      o if o == sta::ABSOLUTE.opcode => sta::ABSOLUTE,
      o if o == sta::ABSOLUTE_X.opcode => sta::ABSOLUTE_X,
      o if o == sta::ABSOLUTE_Y.opcode => sta::ABSOLUTE_Y,
      o if o == sta::INDIRECT_X.opcode => sta::INDIRECT_X,
      o if o == sta::INDIRECT_Y.opcode => sta::INDIRECT_Y,
      o if o == stx::ZERO_PAGE.opcode => stx::ZERO_PAGE,
      o if o == stx::ZERO_PAGE_Y.opcode => stx::ZERO_PAGE_Y,
      o if o == stx::ABSOLUTE.opcode => stx::ABSOLUTE,
      o if o == sty::ZERO_PAGE.opcode => sty::ZERO_PAGE,
      o if o == sty::ZERO_PAGE_X.opcode => sty::ZERO_PAGE_X,
      o if o == sty::ABSOLUTE.opcode => sty::ABSOLUTE,
      o if o == tax::IMPLIED.opcode => tax::IMPLIED,
      o if o == tay::IMPLIED.opcode => tay::IMPLIED,
      o if o == tsx::IMPLIED.opcode => tsx::IMPLIED,
      o if o == txa::IMPLIED.opcode => txa::IMPLIED,
      o if o == txs::IMPLIED.opcode => txs::IMPLIED,
      o if o == tya::IMPLIED.opcode => tya::IMPLIED,
      _ => panic!("instruction not implemented: 0x{:02X}", opcode),
    }
  }
}

pub enum ExtraCycle {
  None,

  /// add 1 to cycles if page boundery is crossed
  Boundary,

  /// add 1 to cycles if branch occurs on same page
  /// add 2 to cycles if branch occurs to different page
  Branch,
}

pub struct Instruction {
  opcode: u8,
  cycles: usize,
  extra_cycle: ExtraCycle,
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
    match self.extra_cycle {
      ExtraCycle::None => self.cycles,
      ExtraCycle::Boundary => {
        let lo = u16::from(memory.read_addr(core.reg.pc));
        let hi = u16::from(memory.read_addr(core.reg.pc + 1));
        let addr = lo | hi << 8;

        self.cycles + is_upper_page_boundary(addr) as usize
      }
      ExtraCycle::Branch => {
        // TODO: branch logic
        unimplemented!();
      }
    }
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

  #[test]
  fn into_byte_test() {
    assert_eq!(into_byte(0), 0b00000000);
    assert_eq!(into_byte(3), 0b00000011);
    assert_eq!(into_byte(-28), 0b11100100);
  }
}
