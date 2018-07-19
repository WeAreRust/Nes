use cpu::Core;
use memory::Memory;

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

#[macro_export]
macro_rules! attr {
    ($p:path) => {
        Attributes {
            opcode: <$p as Instruction>::OPCODE,
            cycles: <$p as Instruction>::CYCLES,
            extra_cycles: <$p as Instruction>::PAGE_BOUNDARY_EXTRA_CYCLES,
        }
    };
}

#[macro_export]
macro_rules! opcode {
    ($p:path) => {
        <$p as Instruction>::OPCODE
    };
}

#[macro_export]
macro_rules! exec {
    ($p:path, $c:ident, $m:ident) => {
        <$p as Instruction>::exec($c, $m)
    };
}

pub trait Instruction {
    const OPCODE: u8;
    const CYCLES: usize;
    const PAGE_BOUNDARY_EXTRA_CYCLES: usize = 0;

    fn exec(core: &mut Core, memory: &mut Memory);
}

pub struct Attributes {
    pub opcode: u8,
    pub cycles: usize,
    pub extra_cycles: usize,
}

// TODO: Dedup lists.
// TODO: Make macros cleaner.

pub fn execute(opcode: u8, core: &mut Core, memory: &mut Memory) {
    // TODO(joshleeb): Move this out of the `execute` function.
    core.reg.pc += 1;

    match opcode {
        opcode!(jmp::Absolute) => exec!(jmp::Absolute, core, memory),
        opcode!(jmp::Indirect) => exec!(jmp::Indirect, core, memory),

        opcode!(lda::Immediate) => exec!(lda::Immediate, core, memory),
        opcode!(lda::ZeroPage) => exec!(lda::ZeroPage, core, memory),
        opcode!(lda::ZeroPageX) => exec!(lda::ZeroPageX, core, memory),
        opcode!(lda::Absolute) => exec!(lda::Absolute, core, memory),
        opcode!(lda::AbsoluteX) => exec!(lda::AbsoluteX, core, memory),
        opcode!(lda::AbsoluteY) => exec!(lda::AbsoluteY, core, memory),
        opcode!(lda::IndirectX) => exec!(lda::IndirectX, core, memory),
        opcode!(lda::IndirectY) => exec!(lda::IndirectY, core, memory),

        opcode!(nop::Implicit) => exec!(nop::Implicit, core, memory),

        _ => unimplemented!(),
    }
}

pub fn get_attrs(opcode: u8) -> Attributes {
    match opcode {
        opcode!(jmp::Absolute) => attr!(jmp::Absolute),
        opcode!(jmp::Indirect) => attr!(jmp::Indirect),

        opcode!(lda::Immediate) => attr!(lda::Immediate),
        opcode!(lda::ZeroPage) => attr!(lda::ZeroPage),
        opcode!(lda::ZeroPageX) => attr!(lda::ZeroPageX),
        opcode!(lda::Absolute) => attr!(lda::Absolute),
        opcode!(lda::AbsoluteX) => attr!(lda::AbsoluteX),
        opcode!(lda::AbsoluteY) => attr!(lda::AbsoluteY),
        opcode!(lda::IndirectX) => attr!(lda::IndirectX),
        opcode!(lda::IndirectY) => attr!(lda::IndirectY),

        opcode!(nop::Implicit) => attr!(nop::Implicit),

        _ => unimplemented!(),
    }
}

pub const CYCLES: [usize; 256] = [
    7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
    6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
    6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
    6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 6, 7,
    2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, 2, 6, 2, 6, 4, 4, 4, 4, 2, 4, 2, 5, 5, 4, 5, 5,
    2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, 2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4,
    2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
    2, 6, 3, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, 2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7,
];
