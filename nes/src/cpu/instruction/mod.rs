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

mod and;

// TODO(benjaminjt): Generate this with a macro, e.g. instruction_set![and::immediate, ...]
fn get_instruction(opcode: u8) -> Instruction {
    const AND_IMMEDIATE_CODE: u8 = and::IMMEDIATE.opcode;
    const AND_ABSOLUTE_CODE: u8 = and::ABSOLUTE.opcode;

    match opcode {
        AND_IMMEDIATE_CODE => and::IMMEDIATE,
        AND_ABSOLUTE_CODE => and::ABSOLUTE,
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
        get_instruction(opcode)
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
  pub fn cycles<T: ReadAddr>(&self, core: &Core, memory: &T) -> usize {
    if !self.page_boundary_extra_cycle {
      return self.cycles;
    }

    let lo = u16::from(memory.read_addr(core.reg.pc));
    let hi = u16::from(memory.read_addr(core.reg.pc + 1));
    let addr = lo | hi << 8;

    self.cycles + is_upper_page_boundary(addr) as usize
  }

  // TODO: test.
  pub fn execute<T: ReadAddr + WriteAddr>(&self, core: &mut Core, memory: &mut T) {
    core.reg.pc += 1;

    pub fn execute<T: ReadAddr + WriteAddr>(&self, core: &mut Core, memory: &mut T) {
        core.reg.pc += 1;

        match self.operation {
            Operation::Implied(op) => op(core),

            Operation::Accumulator(op) => {
                let operand = core.reg.acc;
                op(core, operand);
            }

            Operation::Absolute(op) => {
                let addr = core.absolute_addr(memory);
                let operand = memory.read_addr(addr);
                op(core, operand);
            }

            Operation::AbsoluteX(op) => {
                let addr = core.absolute_addr_x(memory);
                let operand = memory.read_addr(addr);
                op(core, operand);
            }

            Operation::AbsoluteY(op) => {
                let addr = core.absolute_addr_y(memory);
                let operand = memory.read_addr(addr);
                op(core, operand);
            }

            Operation::Immediate(op) => {
                let operand = core.immediate_addr(memory);
                op(core, operand);
            }

            Operation::IndirectX(op) => {
                let addr = core.idx_indirect(memory);
                let operand = memory.read_addr(addr);
                op(core, operand);
            }

            Operation::IndirectY(op) => {
                let addr = core.indirect_idx(memory);
                let operand = memory.read_addr(addr);
                op(core, operand);
            }

            Operation::Relative(op) => {
                let addr = core.relative_addr(memory);
                let operand = memory.read_addr(addr);
                op(core, operand);
            }

            Operation::Zeropage(op) => {
                let addr = core.zero_page_addr(memory);
                let operand = memory.read_addr(addr);
                op(core, operand);
            }

            Operation::ZeropageX(op) => {
                let addr = core.zero_page_addr_x(memory);
                let operand = memory.read_addr(addr);
                op(core, operand);
            }

            Operation::ZeropageY(op) => {
                let addr = core.zero_page_addr_y(memory);
                let operand = memory.read_addr(addr);
                op(core, operand);
            }

            Operation::Indirect(op) => {
                // TODO: Fix this (and figure out where lo_addr comes from...)
                // op(core, memory.read_addr(core.indirect_addr(memory, lo_addr)));
                unimplemented!();
            }
        };
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
