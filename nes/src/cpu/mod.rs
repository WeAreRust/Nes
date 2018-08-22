use clock::Processor;
use cpu::{instruction::Instruction, pipeline::Pipeline, register::Registers};
use memory::{ReadAddr, WriteAddr};
use std::{fmt, u8};

pub mod instruction;
pub mod operation;
mod pipeline;
mod register;

const PAGE_SIZE: u16 = 256;

pub struct Core {
  reg: Registers,
  pipeline: Pipeline,
}

impl Default for Core {
  fn default() -> Self {
    Core::new(Registers::default())
  }
}

impl<T: ReadAddr + WriteAddr> Processor<T> for Core {
  /// Execute a Core
  fn cycle(&mut self, memory: &mut T) {
    if self.pipeline.is_empty() {
      let instr: Instruction = memory.read_addr(self.reg.pc).into();
      self
        .pipeline
        .push(instr.opcode(), instr.cycles(self, memory));
    }
    if let Some(opcode) = self.pipeline.next() {
      println!("{:?}", self);
      println!("Executing: 0x{:02X}", opcode);
      let instr: Instruction = opcode.into();
      instr.execute(self, memory);
    }
  }
}

impl fmt::Debug for Core {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let status: u8 = self.reg.status.into();
    write!(
      f,
      "CPU {{
  A   0x{:02X}
  X   0x{:02X}
  Y   0x{:02X}
  PC  0x{:04X}
  SP  0x{:04X}
  P   0x{:04X}
}}",
      self.reg.acc, self.reg.x_idx, self.reg.y_idx, self.reg.pc, self.reg.stack, status
    )
  }
}

impl Core {
  pub fn new(reg: Registers) -> Self {
    Core {
      reg,
      pipeline: Pipeline::default(),
    }
  }

  /// Get the stack memory address (between 0x0100 and 0x01FF) from the stack counter
  fn get_stack_address(&self) -> u16 {
    0x0100 | (self.reg.stack as u16)
  }

  /// Push a value onto the stack
  fn push_stack(&mut self, memory: &mut WriteAddr, value: u8) {
    memory.write_addr(self.get_stack_address(), value);
    self.reg.stack -= 1; // update the stack address (the stack grows from 0xff to 0x00)
  }

  /// Pop a value from the stack
  fn pop_stack(&mut self, memory: &mut WriteAddr) -> u8 {
    self.reg.stack += 1;
    let value = memory.read_addr(self.get_stack_address());
    value
  }

  /// After a system initialization time of six clock cycles, the mask
  /// interrupt flag will be set and the microprocessor will load the
  /// program counter from the memory vector locations FFFC and
  /// FFFD. This is the start location for program control.
  /// Reference: http://archive.6502.org/datasheets/mos_6500_mpu_nov_1985.pdf
  pub fn reset<T: ReadAddr>(&mut self, memory: &mut T) {
    let pclo: u8 = memory.read_addr(0xFFFC);
    let pchi: u8 = memory.read_addr(0xFFFD);

    self.reg.pc = (pchi as u16) << 8 | pclo as u16;
  }

  /// Immediate addressing allows the use of an 8 bit constant as the arguments to an address.
  fn immediate_addr<T: ReadAddr>(&mut self, memory: &mut T) -> u8 {
    let value = memory.read_addr(self.reg.pc);
    self.reg.pc += 1;

    value
  }

  /// An instruction using zero page addressing mode has only an 8 bit address operand.
  ///
  /// This limits it to addressing only the first 256 bytes of memory (e.g. $0000 to $00FF) where
  /// the most significant byte of the address is always zero. In zero page mode only the least
  /// significant byte of the address is held in the instruction making it shorter by one byte
  /// (important for space saving) and one less memory fetch during execution (important for
  /// speed)
  fn zero_page_addr<T: ReadAddr>(&mut self, memory: &mut T) -> u16 {
    let lo = memory.read_addr(self.reg.pc);
    self.reg.pc += 1;

    lo.into()
  }

  /// The address to be accessed by an instruction using indexed zero page addressing is
  /// calculated by taking the sum of the zero page address from memory, and the value of the X
  /// Index register
  ///
  /// For example if the X register contains $0F and the instruction LDA $80,X is executed then
  /// the accumulator will be loaded from $008F (e.g. $80 + $0F => $8F).
  ///
  /// The address calculation wraps around if the sum of the base address and the register exceed
  /// $FF. If we repeat the last example but with $FF in the X register then the accumulator will
  /// be loaded from $007F (e.g. $80 + $FF => $7F) and not $017F.
  fn zero_page_addr_x<T: ReadAddr>(&mut self, memory: &mut T) -> u16 {
    let lo = memory.read_addr(self.reg.pc).wrapping_add(self.reg.x_idx);
    self.reg.pc += 1;

    lo.into()
  }

  /// The address to be accessed by an instruction using indexed zero page addressing is
  /// calculated by taking the sum of the zero page address from memory, and the value of the Y
  /// Index register
  ///
  /// This mode can only be used with the LDX and STX instructions.
  ///
  /// The address calculation wraps around if the sum of the base address and the register exceed
  /// $FF. If we repeat the last example but with $FF in the X register then the accumulator will
  /// be loaded from $007F (e.g. $80 + $FF => $7F) and not $017F.
  fn zero_page_addr_y<T: ReadAddr>(&mut self, memory: &mut T) -> u16 {
    let lo = memory.read_addr(self.reg.pc).wrapping_add(self.reg.y_idx);
    self.reg.pc += 1;

    lo.into()
  }

  /// The address to be accessed by an instruction using relative addressing is calculated by a
  /// signed 8 bit relative offset.
  ///
  /// The relative offset is in the range [-128, +127] and is added to the program counter. The
  /// program counter itself is incremented during the instruction execution, so the distance to
  /// jump is truly in the range [-126, +129].
  fn relative_addr<T: ReadAddr>(&mut self, memory: &mut T) -> u16 {
    let offset = memory.read_addr(self.reg.pc).into();
    self.reg.pc += 2;

    self.reg.pc.wrapping_add(offset)
  }

  /// Absolute addressing allows the use of an 16 bit address to identify the target location.
  fn absolute_addr<T: ReadAddr>(&mut self, memory: &mut T) -> u16 {
    let lo = u16::from(memory.read_addr(self.reg.pc));
    let hi = u16::from(memory.read_addr(self.reg.pc + 1));
    self.reg.pc += 2;

    lo | hi << 8
  }

  /// The address to be accessed by an instruction using X register indexed absolute addressing
  /// is computed by taking the sum of the 16 bit address from the instruction, the value of the
  /// X Index register
  ///
  /// For example if X contains $92 then an STA $2000,X instruction will store the accumulator at
  /// $2092 (e.g. $2000 + $92).
  fn absolute_addr_x<T: ReadAddr>(&mut self, memory: &mut T) -> u16 {
    let lo = u16::from(memory.read_addr(self.reg.pc));
    let hi = u16::from(memory.read_addr(self.reg.pc + 1));
    self.reg.pc += 2;

    (lo | hi << 8).wrapping_add(self.reg.x_idx.into())
  }

  /// The address to be accessed by an instruction using Y register indexed absolute addressing
  /// is computed by taking the sum of the 16 bit address from the instruction, the value of the
  /// Y Index register
  fn absolute_addr_y<T: ReadAddr>(&mut self, memory: &mut T) -> u16 {
    let lo = u16::from(memory.read_addr(self.reg.pc));
    let hi = u16::from(memory.read_addr(self.reg.pc + 1));
    self.reg.pc += 2;

    (lo | hi << 8).wrapping_add(self.reg.y_idx.into())
  }

  /// JMP is the only 6502 instruction to support indirection. The instruction contains a 16 bit
  /// address which identifies the location of the least significant byte of another 16 bit
  /// memory address which is the real target of the instruction.
  ///
  /// The 6502 process contains a bug specifically for indirect jumps that needs to be
  /// reproduced. If address $3000 contains $40, $30FF contains $80, and $3100 contains $50, the
  /// result of JMP ($30FF) will be a transfer of control to $4080 rather than $5080 as you
  /// intended i.e. the 6502 took the low byte of the address from $30FF and the high byte from
  /// $3000.
  fn indirect_addr<T: ReadAddr>(&mut self, memory: &mut T, lo_addr: u16) -> u16 {
    let hi_addr = if instruction::is_upper_page_boundary(lo_addr) {
      (lo_addr / PAGE_SIZE) * PAGE_SIZE
    } else {
      lo_addr + 1
    };

    let lo = u16::from(memory.read_addr(lo_addr));
    let hi = u16::from(memory.read_addr(hi_addr));

    lo | hi << 8
  }

  /// Indexed indirect addressing is normally used in conjunction with a table of address held on
  /// zero page. The address of the table is taken from the instruction and the X register added
  /// to it (with zero page wrap around) to give the location of the least significant byte of
  /// the target address
  ///
  /// TODO: Test ZEROPAGE wrap wround.
  ///
  /// Also seen in spec sheets as `Indirect,X`.
  fn idx_indirect<T: ReadAddr>(&mut self, memory: &mut T) -> u16 {
    let addr = u16::from(memory.read_addr(self.reg.pc).wrapping_add(self.reg.x_idx));
    self.reg.pc += 1;

    let lo = u16::from(memory.read_addr(addr));
    let hi = u16::from(memory.read_addr(addr + 1));

    lo | hi << 8
  }

  /// Indirect indirect addressing is the most common indirection mode used on the 6502. In
  /// instruction contains the zero page location of the least significant byte of 16 bit
  /// address. The Y register is dynamically added to this value to generated the actual target
  /// address for operation
  ///
  /// Also seen in spec sheets as `Indirect,Y`.
  fn indirect_idx<T: ReadAddr>(&mut self, memory: &mut T) -> u16 {
    let addr = u16::from(memory.read_addr(self.reg.pc));
    self.reg.pc += 1;

    let lo = u16::from(memory.read_addr(addr));
    let hi = u16::from(memory.read_addr(addr + 1));

    (lo | hi << 8).wrapping_add(self.reg.y_idx.into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use cpu::{instruction::Instruction, register::Registers};
  use memory::block::BlockMemory;

  #[test]
  fn get_stack_address() {
    let mut core = Core::new(Registers::empty());
    core.reg.stack = 0xf0;
    assert_eq!(core.get_stack_address(), 0x01f0);
  }

  #[test]
  fn push_stack() {
    let mut core = Core::new(Registers::empty());
    let mut memory = BlockMemory::with_size(0x0200);
    core.reg.stack = 0xff; // init stack
    core.push_stack(&mut memory, 0x05);
    assert_eq!(core.reg.stack, 0xfe);
    assert_eq!(memory.read_addr(0x01ff), 0x05);
  }

  #[test]
  fn pop_stack() {
    let mut core = Core::new(Registers::empty());
    let mut memory = BlockMemory::with_size(0x0200);
    memory.write_addr(0x01f0, 0x05);
    core.reg.stack = 0xf0 - 1;
    assert_eq!(core.pop_stack(&mut memory), 0x05);
    assert_eq!(core.reg.stack, 0xf0);
  }

  #[test]
  fn processor_cycle() {
    let lda = Instruction::from(0xa9); // LDA Immedate
    let jmp = Instruction::from(0x4c); // JMP Absolute

    let mut memory = BlockMemory::with_bytes(vec![
      // LDA #$5f
      lda.opcode(),
      0x55,
      //JMP $5597
      jmp.opcode(),
      0x97,
      0x55,
    ]);
    let mut core = Core::new(Registers::empty());

    core.cycle(&mut memory);
    assert_eq!(core.pipeline.rem_cycles, lda.base_cycles() - 1);

    core.cycle(&mut memory);
    assert_eq!(core.pipeline, Pipeline::new(None, 0));
    assert_eq!(core.reg.acc, 0x55);

    core.cycle(&mut memory);
    assert_eq!(core.pipeline.rem_cycles, jmp.base_cycles() - 1);

    core.cycle(&mut memory);
    assert_eq!(core.pipeline.rem_cycles, jmp.base_cycles() - 2);
    assert_eq!(core.reg.pc, 2);

    core.cycle(&mut memory);
    assert!(core.pipeline.opcode.is_none());
    assert_eq!(core.pipeline.rem_cycles, jmp.base_cycles() - 3);
    assert_eq!(core.reg.pc, 0x5597);
  }

  #[test]
  fn reset() {
    let mut v = vec![0x00u8; 0xffff];
    v[0xfffc] = 0xaa;
    v[0xfffd] = 0xbb;
    let mut memory = BlockMemory::with_bytes(v);
    let mut core = Core::new(Registers::empty());
    core.reg.pc = 1;

    core.reset(&mut memory);

    assert_eq!(core.reg.pc, 0xbbaa);
  }

  #[test]
  fn immediate_address() {
    let mut memory = BlockMemory::with_bytes(vec![0x00, 0xff]);
    let mut core = Core::new(Registers::empty());
    core.reg.pc = 1;

    let addr = core.immediate_addr(&mut memory);
    assert_eq!(addr, 0xff);
    assert_eq!(core.reg.pc, 2);
  }

  #[test]
  fn zero_page_address() {
    let mut memory = BlockMemory::with_bytes(vec![0x97]);
    let mut core = Core::new(Registers::empty());

    let addr = core.zero_page_addr(&mut memory);
    assert_eq!(addr, 0x0097);
    assert_eq!(core.reg.pc, 1);
  }

  #[test]
  fn zero_page_address_x() {
    let mut memory = BlockMemory::with_bytes(vec![0x97]);
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 1;

    let addr = core.zero_page_addr_x(&mut memory);
    assert_eq!(addr, 0x0098);
    assert_eq!(core.reg.pc, 1);
  }

  #[test]
  fn zero_page_address_x_overflow() {
    let mut memory = BlockMemory::with_bytes(vec![0xff]);
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 3;

    let addr = core.zero_page_addr_x(&mut memory);
    assert_eq!(addr, 0x0002);
    assert_eq!(core.reg.pc, 1);
  }

  #[test]
  fn zero_page_address_y() {
    let mut memory = BlockMemory::with_bytes(vec![0x97]);
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 1;

    let addr = core.zero_page_addr_y(&mut memory);
    assert_eq!(addr, 0x0098);
    assert_eq!(core.reg.pc, 1);
  }

  #[test]
  fn zero_page_address_y_overflow() {
    let mut memory = BlockMemory::with_bytes(vec![0xff]);
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 3;

    let addr = core.zero_page_addr_y(&mut memory);
    assert_eq!(addr, 0x0002);
    assert_eq!(core.reg.pc, 1);
  }

  #[test]
  fn relative_address() {
    let mut memory = BlockMemory::with_bytes(vec![0x00, 0x00, 0x12]);
    let mut core = Core::new(Registers::empty());
    core.reg.pc = 2;

    let addr = core.relative_addr(&mut memory);
    assert_eq!(addr, core.reg.pc + 0x12);
    assert_eq!(core.reg.pc, 4);
  }

  #[test]
  fn relative_address_overflow() {
    let mut memory = BlockMemory::with_bytes(vec![0x00, 0x00, 0x80]);
    let mut core = Core::new(Registers::empty());
    core.reg.pc = 2;

    let addr = core.relative_addr(&mut memory);
    assert_eq!(addr, 256 - 0x80 + core.reg.pc);
    assert_eq!(core.reg.pc, 4);
  }

  #[test]
  fn absolute_address() {
    let mut memory = BlockMemory::with_bytes(vec![0x97, 0x55]);
    let mut core = Core::new(Registers::empty());

    let addr = core.absolute_addr(&mut memory);
    assert_eq!(addr, 0x5597);
    assert_eq!(core.reg.pc, 2);
  }

  #[test]
  fn absolute_address_x() {
    let mut memory = BlockMemory::with_bytes(vec![0x97, 0x55]);
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 2;

    let addr = core.absolute_addr_x(&mut memory);
    assert_eq!(addr, 0x5599);
    assert_eq!(core.reg.pc, 2);
  }

  #[test]
  fn absolute_address_x_overflow() {
    let mut memory = BlockMemory::with_bytes(vec![0xff, 0xff]);
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 2;

    let addr = core.absolute_addr_x(&mut memory);
    assert_eq!(addr, 0x0001);
    assert_eq!(core.reg.pc, 2);
  }

  #[test]
  fn absolute_address_y() {
    let mut memory = BlockMemory::with_bytes(vec![0x97, 0x55]);
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 2;

    let addr = core.absolute_addr_y(&mut memory);
    assert_eq!(addr, 0x5599);
    assert_eq!(core.reg.pc, 2);
  }

  #[test]
  fn absolute_address_y_overflow() {
    let mut memory = BlockMemory::with_bytes(vec![0xff, 0xff]);
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 2;

    let addr = core.absolute_addr_y(&mut memory);
    assert_eq!(addr, 0x0001);
    assert_eq!(core.reg.pc, 2);
  }

  #[test]
  fn indirect_address() {
    let mut bytes = vec![0; 65536];
    bytes[0x30fe] = 0x80;
    bytes[0x30ff] = 0x50;

    let mut memory = BlockMemory::with_bytes(bytes);
    let mut core = Core::new(Registers::empty());

    let addr = core.indirect_addr(&mut memory, 0x30fe);
    assert_eq!(addr, 0x5080);
  }

  #[test]
  fn indirect_address_overflow() {
    let mut bytes = vec![0; 65536];
    bytes[0x30ff] = 0x80;
    bytes[0x3100] = 0x50;
    bytes[0x3000] = 0x40;

    let mut memory = BlockMemory::with_bytes(bytes);
    let mut core = Core::new(Registers::empty());

    let addr = core.indirect_addr(&mut memory, 0x30ff);
    assert_eq!(addr, 0x4080);
  }

  #[test]
  fn index_indirect() {
    let mut memory = BlockMemory::with_bytes(vec![0x01, 0xff, 0xff, 0x97, 0x55]);
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 2;

    let addr = core.idx_indirect(&mut memory);
    assert_eq!(addr, 0x5597);
    assert_eq!(core.reg.pc, 1);
  }

  #[test]
  fn index_indirect_overflow() {
    let mut memory = BlockMemory::with_bytes(vec![0xff, 0x97, 0x55]);
    let mut core = Core::new(Registers::empty());
    core.reg.x_idx = 2;

    let addr = core.idx_indirect(&mut memory);
    assert_eq!(addr, 0x5597);
    assert_eq!(core.reg.pc, 1);
  }

  #[test]
  fn indirect_index() {
    let mut memory = BlockMemory::with_bytes(vec![0x03, 0xff, 0xff, 0x97, 0x55]);
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 2;

    let addr = core.indirect_idx(&mut memory);
    assert_eq!(addr, 0x5599);
    assert_eq!(core.reg.pc, 1);
  }

  #[test]
  fn indirect_index_overflow() {
    let mut memory = BlockMemory::with_bytes(vec![0x01, 0xff, 0xff]);
    let mut core = Core::new(Registers::empty());
    core.reg.y_idx = 2;

    let addr = core.indirect_idx(&mut memory);
    assert_eq!(addr, 0x0001);
    assert_eq!(core.reg.pc, 1);
  }
}
