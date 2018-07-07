use cpu::Core;

/// Execute the opcode and return the number of cycles.
pub fn execute(cpu: &mut Core, opcode: u8) -> usize {
    match opcode {
        0x4c => jump_abs(cpu),
        0x6c => jump_indr(cpu),
        _ => 0,
    }
}

/// Jump to absolute address (JMP).
///
/// Flags affected: None
fn jump_abs(cpu: &mut Core) -> usize {
    cpu.reg.pc = cpu.abs_addr();
    3
}

/// Jump to indirect address (JMP).
///
/// Flags affected: None
///
/// An indirect jump must never use a vector beginning on the last byte of a page. If this
/// occurs then the low byte should be as expected, and the high byte should wrap to the start
/// of the page. See http://www.6502.org/tutorials/6502opcodes.html#JMP for details.
fn jump_indr(cpu: &mut Core) -> usize {
    cpu.reg.pc = cpu.indr_addr();
    5
}
