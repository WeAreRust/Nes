use cpu::{instruction::Instruction, operation::Operation, Core};

/// AND operand with accumulator
///
/// Flags affected: N, Z
#[inline(always)]
fn and(core: &mut Core, operand: u8) {
    core.reg.acc &= operand;
    core.reg.status.set_negative(core.reg.acc);
    core.reg.status.set_zero(core.reg.acc);
}

/// AND memory with accumulator immediate
///
/// Flags affected: N, Z
pub const IMMEDIATE: Instruction = Instruction {
    opcode: 0x29,
    cycles: 2,
    page_boundary_extra_cycle: false,
    operation: Operation::Immediate(&and),
};

/// AND memory with accumulator zero page
///
/// Flags affected: N, Z
pub const ZERO_PAGE: Instruction = Instruction {
    opcode: 0x25,
    cycles: 3,
    page_boundary_extra_cycle: false,
    operation: Operation::Zeropage(&and),
};

/// AND memory with accumulator zero page X
///
/// Flags affected: N, Z
pub const ZERO_PAGE_X: Instruction = Instruction {
    opcode: 0x35,
    cycles: 4,
    page_boundary_extra_cycle: false,
    operation: Operation::ZeropageX(&and),
};

/// AND memory with accumulator absolute
///
/// Flags affected: N, Z
pub const ABSOLUTE: Instruction = Instruction {
    opcode: 0x2d,
    cycles: 4,
    page_boundary_extra_cycle: false,
    operation: Operation::Absolute(&and),
};

/// AND memory with accumulator absolute X
///
/// Flags affected: N, Z
pub const ABSOLUTE_X: Instruction = Instruction {
    opcode: 0x3d,
    cycles: 4,
    page_boundary_extra_cycle: true,
    operation: Operation::AbsoluteX(&and),
};

/// AND memory with accumulator absolute Y
///
/// Flags affected: N, Z
pub const ABSOLUTE_Y: Instruction = Instruction {
    opcode: 0x39,
    cycles: 4,
    page_boundary_extra_cycle: true,
    operation: Operation::AbsoluteY(&and),
};

/// AND memory with accumulator indirect X
///
/// Flags affected: N, Z
pub const INDIRECT_X: Instruction = Instruction {
    opcode: 0x21,
    cycles: 6,
    page_boundary_extra_cycle: false,
    operation: Operation::IndirectX(&and),
};

/// AND memory with accumulator indirect Y
///
/// Flags affected: N, Z
pub const INDIRECT_Y: Instruction = Instruction {
    opcode: 0x31,
    cycles: 5,
    page_boundary_extra_cycle: true,
    operation: Operation::IndirectY(&and),
};

// TODO(benjaminjt): Fix tests
