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

#[cfg(test)]
mod tests {
    use super::*;

    use asm6502::assemble;
    use memory::NesMemorySpace;

    #[test]
    fn jump_absolute() {
        let mut cpu = cpu_with_asm("JMP $5597");
        let cycles = jump_abs(&mut cpu);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.reg.pc, 0x5597);
    }

    // TODO(jsohleeb): Clean up helper function.
    fn cpu_with_asm(asm: &str) -> Core {
        let asm = asm.as_bytes();
        let mut buf = Vec::with_capacity(65536);
        assemble(asm, &mut buf).unwrap();

        println!("{:x?}", buf);

        for i in 3..65536 {
            buf.push(0);
        }

        let mut data = [0; 65536];
        data.copy_from_slice(buf.as_slice());

        let mut cpu = Core::with_data(data);
        cpu.reg.pc = 1;

        cpu
    }
}
