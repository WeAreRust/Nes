use cycles::Cycles;
use instruction::Instruction;

pub fn get_instruction(opcode: u8) -> (Instruction, Cycles) {
  match opcode {
    0x0A => (Instruction::Accumulator(&asl), Cycles::N(2)),
    0x0E => (Instruction::Absolute(&asl), Cycles::N(6)),
    _ => unimplemented!(),
  }
}

/// ASL  Shift Left One Bit (Memory or Accumulator)
///
/// C <- [76543210] <- 0
///
/// Flags affected: N, Z, C
fn asl(core: &mut Core, memory: &mut Memory, mode: impl mode::ValueMode) {
  let result = (mode.read(core, memory) as u16) << 1;
  mode.write(core, memory, result as u8);
  // TODO: Update flags
}
