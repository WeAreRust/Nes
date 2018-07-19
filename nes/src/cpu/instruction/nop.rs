use cpu::{instruction::Execute, Core};
use memory::Memory;

/// No Operation
///
/// Flags affected: None
#[derive(Execute)]
#[opcode = 0xea]
#[cycles = 2]
pub struct Implicit;

fn implicit(_core: &mut Core, _memory: &mut Memory) {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use cpu::{
//         instruction,
//         register::{Registers, StatusFlags},
//     };
//     use memory::{Memory, ReadAddr};

//     #[test]
//     #[ignore]
//     fn nop() {
//         let mut memory = Memory::with_bytes(nes_asm!("NOP"));
//         let mut cpu = Core::new(Registers::empty());

//         let opcode = memory.read_addr(0);
//         assert_eq!(opcode, <Implicit as Execute>::OPCODE);

//         instruction::execute(opcode, &mut cpu, &mut memory);
//         assert_eq!(cpu.reg.status, StatusFlags::empty());
//     }
// }
