use cpu::{instruction::Execute, Core};
use memory::{Memory, ReadAddr};

/// Load accumulator immediate
///
/// Flags affected: N, Z
#[derive(Execute)]
#[opcode = 0xa9]
#[cycles = 2]
pub struct Immediate;

fn immediate(core: &mut Core, memory: &mut Memory) {
    let value = core.immediate_addr(memory);
    core.reg.acc = value;
    update_flags(core);
}

/// Load accumulator zero page
///
/// Flags affected: N, Z
#[derive(Execute)]
#[opcode = 0xa5]
#[cycles = 3]
pub struct ZeroPage;

fn zero_page(core: &mut Core, memory: &mut Memory) {
    let addr = core.zero_page_addr(memory);
    core.reg.acc = memory.read_addr(addr);
    update_flags(core);
}

/// Load accumulator zero page X
///
/// Flags affected: N, Z
#[derive(Execute)]
#[opcode = 0xb5]
#[cycles = 2]
pub struct ZeroPageX;

fn zero_page_x(core: &mut Core, memory: &mut Memory) {
    let addr = core.zero_page_addr_x(memory);
    core.reg.acc = memory.read_addr(addr);
    update_flags(core);
}

/// Load accumulator absolute
///
/// Flags affected: N, Z
#[derive(Execute)]
#[opcode = 0xad]
#[cycles = 4]
pub struct Absolute;

fn absolute(core: &mut Core, memory: &mut Memory) {
    let addr = core.absolute_addr(memory);
    core.reg.acc = memory.read_addr(addr);
    update_flags(core);
}

/// Load accumulator absolute X
///
/// Flags affected: N, Z
/// TODO: test normal execution
/// TODO: test page boundary execution
#[derive(Execute)]
#[opcode = 0xbd]
#[cycles = 4]
#[page_boundary_extra_cycle]
pub struct AbsoluteX;

fn absolute_x(core: &mut Core, memory: &mut Memory) {
    let addr = core.absolute_addr_x(memory);
    core.reg.acc = memory.read_addr(addr);
    update_flags(core);
}

/// Load accumulator absolute Y
///
/// Flags affected: N, Z
/// TODO: test normal execution
/// TODO: test page boundary execution
#[derive(Execute)]
#[opcode = 0xb9]
#[cycles = 4]
#[page_boundary_extra_cycle]
pub struct AbsoluteY;

fn absolute_y(core: &mut Core, memory: &mut Memory) {
    let addr = core.absolute_addr_y(memory);
    core.reg.acc = memory.read_addr(addr);
    update_flags(core);
}

/// Load accumulator indirect X
///
/// Flags affected: N, Z
/// TODO: test
#[derive(Execute)]
#[opcode = 0xa1]
#[cycles = 6]
pub struct IndirectX;

fn indirect_x(core: &mut Core, memory: &mut Memory) {
    let addr = core.idx_indirect(memory);
    core.reg.acc = memory.read_addr(addr);
    update_flags(core);
}

/// Load accumulator indirect Y
///
/// Flags affected: N, Z
/// TODO: test normal execution
/// TODO: test page boundary execution
#[derive(Execute)]
#[opcode = 0xb1]
#[cycles = 2]
#[page_boundary_extra_cycle]
pub struct IndirectY;

fn indirect_y(core: &mut Core, memory: &mut Memory) {
    let addr = core.indirect_idx(memory);
    core.reg.acc = memory.read_addr(addr);
    update_flags(core);
}

fn update_flags(core: &mut Core) {
    core.reg.status.set_negative(core.reg.acc);
    core.reg.status.set_zero(core.reg.acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    use cpu::{
        instruction::Instruction,
        register::{Registers, StatusFlags},
    };
    use memory::ReadAddr;

    #[test]
    fn load_accumulator_immediate() {
        let mut memory = Memory::with_bytes(nes_asm!("LDA #$5f"));
        let mut core = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, <Immediate as Execute>::OPCODE);

        Instruction::from(opcode).execute(&mut core, &mut memory);
        assert_eq!(core.reg.acc, 0x5f);
        assert_eq!(core.reg.status, StatusFlags::empty());
    }

    #[test]
    fn load_accumulator_zero_page() {
        let mut bytes = nes_asm!("LDA $03");
        bytes.extend(vec![0xff, 0x44]);

        let mut memory = Memory::with_bytes(bytes);
        let mut core = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, <ZeroPage as Execute>::OPCODE);

        Instruction::from(opcode).execute(&mut core, &mut memory);
        assert_eq!(core.reg.acc, 0x44);
        assert_eq!(core.reg.status, StatusFlags::empty());
    }

    #[test]
    fn load_accumulator_absolute() {
        let mut bytes = nes_asm!("LDA $0004");
        bytes.extend(vec![0xff, 0x44]);

        let mut memory = Memory::with_bytes(bytes);
        let mut core = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, <Absolute as Execute>::OPCODE);

        Instruction::from(opcode).execute(&mut core, &mut memory);
        assert_eq!(core.reg.acc, 0x44);
        assert_eq!(core.reg.status, StatusFlags::empty());
    }

    #[test]
    fn load_accumulator_zero_flag() {
        let mut memory = Memory::with_bytes(nes_asm!("LDA #$00"));
        let mut core = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, <Immediate as Execute>::OPCODE);

        Instruction::from(opcode).execute(&mut core, &mut memory);
        assert_eq!(core.reg.acc, 0x00);
        assert_eq!(core.reg.status, StatusFlags::Z_FLAG);
    }

    #[test]
    fn load_accumulator_negative_flag() {
        let mut memory = Memory::with_bytes(nes_asm!("LDA #$98"));
        let mut core = Core::new(Registers::empty());

        let opcode = memory.read_addr(0);
        assert_eq!(opcode, <Immediate as Execute>::OPCODE);

        Instruction::from(opcode).execute(&mut core, &mut memory);
        assert_eq!(core.reg.acc, 0b10011000);
        assert_eq!(core.reg.status, StatusFlags::N_FLAG);
    }
}
