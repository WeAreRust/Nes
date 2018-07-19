use cpu::Core;
use memory::Memory;
use std::convert::From;

#[macro_export]
macro_rules! instruction_match {
    ($op:ident, $fn:ident, $($a:ident),*) => {
        match $op {
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

            _ => unimplemented!(),
        }
    };
    ($op:ident, $fn:ident) => (instruction_match!($op, $fn,))
}

#[cfg(test)]
#[macro_export]
macro_rules! nes_asm {
    ($e:expr) => {{
        let mut buf = vec![];
        // We push a newline into the bytes array because of a known issue in asm6502
        ::asm6502::assemble(format!("{}\n", $e).as_bytes(), &mut buf).unwrap();
        buf
    }};
}

mod jmp;
mod lda;
mod nop;

trait Execute {
    const OPCODE: u8;
    const CYCLES: usize;
    const PAGE_BOUNDARY_EXTRA_CYCLES: bool = false;

    fn exec(core: &mut Core, memory: &mut Memory);

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
    pub fn opcode(&self) -> u8 {
        self.opcode
    }

    pub fn cycles(&self, _core: &Core, _memory: &Memory) -> usize {
        if !self.page_boundary_extra_cycle {
            return self.cycles;
        }

        // TODO: Implement properly: Check if across page boundary.
        self.cycles
    }

    pub fn execute(&self, core: &mut Core, memory: &mut Memory) {
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
