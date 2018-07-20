use cpu::{instruction::Execute, Core};
use memory::ReadAddr;

/// Jump absolute
///
/// Flags affected: None
#[derive(Execute)]
#[opcode = 0x4c]
#[cycles = 3]
pub struct Absolute;

#[inline(always)]
fn absolute<T: ReadAddr>(core: &mut Core, memory: &mut T) {
    core.reg.pc = core.absolute_addr(memory);
}

/// Jump indirect
///
/// Flags affected: None
///
/// An indirect jump must never use a vector beginning on the last byte of a page. If this
/// occurs then the low byte should be as expected, and the high byte should wrap to the start
/// of the page. See http://www.6502.org/tutorials/6502opcodes.html#JMP for details.
#[derive(Execute)]
#[opcode = 0x6c]
#[cycles = 5]
pub struct Indirect;

#[inline(always)]
fn indirect<T: ReadAddr>(core: &mut Core, memory: &mut T) {
    let arg_addr = core.absolute_addr(memory);
    core.reg.pc = core.indirect_addr(memory, arg_addr);
}

#[cfg(test)]
mod tests {
    use super::*;

    use cpu::{instruction::Instruction, register::Registers};
    use memory::{Memory, ReadAddr};

    #[test]
    fn jump_absolute() {
        let mut memory = Memory::with_bytes(nes_asm!("JMP $5597"));
        let mut core = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, <Absolute as Execute>::OPCODE);

        Instruction::from(opcode).execute(&mut core, &mut memory);
        assert_eq!(core.reg.pc, 0x5597);
    }

    #[test]
    fn jump_indirect() {
        let mut bytes = nes_asm!("JMP ($0004)");
        bytes.extend(vec![0xff, 0x97, 0x55]);

        let mut memory = Memory::with_bytes(bytes);
        let mut core = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, <Indirect as Execute>::OPCODE);

        Instruction::from(opcode).execute(&mut core, &mut memory);
        assert_eq!(core.reg.pc, 0x5597);
    }
}
