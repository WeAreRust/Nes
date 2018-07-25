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
mod jmp;
mod lda;
mod nop;

#[macro_export]
macro_rules! instruction_match {
    ($op:ident, $fn:ident, $($a:ident),*) => {
        match $op {
            <and::Immediate as Execute>::OPCODE => <and::Immediate as Execute>::$fn($($a),*),
            <and::ZeroPage as Execute>::OPCODE => <and::ZeroPage as Execute>::$fn($($a),*),
            <and::ZeroPageX as Execute>::OPCODE => <and::ZeroPageX as Execute>::$fn($($a),*),
            <and::Absolute as Execute>::OPCODE => <and::Absolute as Execute>::$fn($($a),*),
            <and::AbsoluteX as Execute>::OPCODE => <and::AbsoluteX as Execute>::$fn($($a),*),
            <and::AbsoluteY as Execute>::OPCODE => <and::AbsoluteY as Execute>::$fn($($a),*),
            <and::IndirectX as Execute>::OPCODE => <and::IndirectX as Execute>::$fn($($a),*),
            <and::IndirectY as Execute>::OPCODE => <and::IndirectY as Execute>::$fn($($a),*),

            <jmp::Absolute as Execute>::OPCODE => <jmp::Absolute as Execute>::$fn($($a),*),
            <jmp::Indirect as Execute>::OPCODE => <jmp::Indirect as Execute>::$fn($($a),*),

            <lda::Immediate as Execute>::OPCODE => <lda::Immediate as Execute>::$fn($($a),*),
            <lda::ZeroPage as Execute>::OPCODE => <lda::ZeroPage as Execute>::$fn($($a),*),
            <lda::ZeroPageX as Execute>::OPCODE => <lda::ZeroPageX as Execute>::$fn($($a),*),
            <lda::Absolute as Execute>::OPCODE => <lda::Absolute as Execute>::$fn($($a),*),
            <lda::AbsoluteX as Execute>::OPCODE => <lda::AbsoluteX as Execute>::$fn($($a),*),
            <lda::AbsoluteY as Execute>::OPCODE => <lda::AbsoluteY as Execute>::$fn($($a),*),
            <lda::IndirectX as Execute>::OPCODE => <lda::IndirectX as Execute>::$fn($($a),*),
            <lda::IndirectY as Execute>::OPCODE => <lda::IndirectY as Execute>::$fn($($a),*),

            <nop::Implicit as Execute>::OPCODE => <nop::Implicit as Execute>::$fn($($a),*),

            _ => panic!("not yet implemented ({:?})", $op),
        }
    };
    ($op:ident, $fn:ident) => (instruction_match!($op, $fn,))
}

trait Execute {
    const OPCODE: u8;
    const CYCLES: usize;
    const PAGE_BOUNDARY_EXTRA_CYCLES: bool = false;

    fn exec<T: ReadAddr + WriteAddr>(core: &mut Core, memory: &mut T);

    #[inline(always)]
    fn to_instruction() -> Instruction {
        Instruction {
            opcode: Self::OPCODE,
            cycles: Self::CYCLES,
            page_boundary_extra_cycle: Self::PAGE_BOUNDARY_EXTRA_CYCLES,
        }
    }
}

pub struct Instruction {
    opcode: u8,
    cycles: usize,
    page_boundary_extra_cycle: bool,
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

        let opcode = self.opcode;
        instruction_match!(opcode, exec, core, memory)
    }
}

impl From<u8> for Instruction {
    fn from(opcode: u8) -> Self {
        instruction_match!(opcode, to_instruction)
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
