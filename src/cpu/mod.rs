use self::register::Registers;

use clock::Processor;
use memory::{Memory, ReadAddr};
use std::u8;

mod instruction;
mod register;

pub struct Core {
    pub reg: Registers,
}

impl Processor for Core {
    fn cycle(&mut self, memory: &mut Memory) {
        let opcode = memory.read_addr(self.reg.pc);

        // TODO(joshleeb): Timing using cycles. Execute after cycles are completed?
        // let cycles = instruction::CYCLES[opcode as usize];

        self.execute(opcode, memory);
    }
}

impl Core {
    pub fn new(reg: Registers) -> Self {
        Core { reg }
    }

    /// Immediate addressing allows the use of an 8 bit constant as the arguments to an address.
    fn immediate_addr(&mut self, memory: &mut Memory) -> u8 {
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
    fn zero_page_addr(&mut self, memory: &mut Memory) -> u16 {
        let lo = memory.read_addr(self.reg.pc) as u16;
        self.reg.pc += 1;

        lo
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
    fn zero_page_addr_x(&mut self, memory: &mut Memory) -> u16 {
        let lo = memory.read_addr(self.reg.pc).wrapping_add(self.reg.x_idx);
        self.reg.pc += 1;

        lo as u16
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
    fn zero_page_addr_y(&mut self, memory: &mut Memory) -> u16 {
        let lo = memory.read_addr(self.reg.pc).wrapping_add(self.reg.y_idx);
        self.reg.pc += 1;

        lo as u16
    }

    /// The address to be accessed by an instruction using relative addressing is calculated by a
    /// signed 8 bit relative offset.
    ///
    /// The relative offset is in the range [-128, +127] and is added to the program counter. The
    /// program counter itself is incremented during the instruction execution, so the distance to
    /// jump is truly in the range [-126, +129].
    fn relative_addr(&mut self, memory: &mut Memory) -> u16 {
        let offset = memory.read_addr(self.reg.pc) as u16;
        self.reg.pc += 2;

        self.reg.pc.wrapping_add(offset)
    }

    /// Absolute addressing allows the use of an 16 bit address to identify the target location.
    fn absolute_addr(&mut self, memory: &mut Memory) -> u16 {
        let lo = memory.read_addr(self.reg.pc) as u16;
        let hi = memory.read_addr(self.reg.pc + 1) as u16;
        self.reg.pc += 2;

        lo | hi << 8
    }

    /// The address to be accessed by an instruction using X register indexed absolute addressing
    /// is computed by taking the sum of the 16 bit address from the instruction, the value of the
    /// X Index register
    ///
    /// For example if X contains $92 then an STA $2000,X instruction will store the accumulator at
    /// $2092 (e.g. $2000 + $92).
    fn absolute_addr_x(&mut self, memory: &mut Memory) -> u16 {
        let lo = memory.read_addr(self.reg.pc) as u16;
        let hi = memory.read_addr(self.reg.pc + 1) as u16;
        self.reg.pc += 2;

        (lo | hi << 8).wrapping_add(self.reg.x_idx as u16)
    }

    /// The address to be accessed by an instruction using Y register indexed absolute addressing
    /// is computed by taking the sum of the 16 bit address from the instruction, the value of the
    /// Y Index register
    fn absolute_addr_y(&mut self, memory: &mut Memory) -> u16 {
        let lo = memory.read_addr(self.reg.pc) as u16;
        let hi = memory.read_addr(self.reg.pc + 1) as u16;
        self.reg.pc += 2;

        (lo | hi << 8).wrapping_add(self.reg.y_idx as u16)
    }

    /// Indexed indirect addressing is normally used in conjunction with a table of address held on
    /// zero page. The address of the table is taken from the instruction and the X register added
    /// to it (with zero page wrap around) to give the location of the least significant byte of
    /// the target address
    ///
    /// Also seen in spec sheets as `Indirect,X`.
    fn idx_indirect(&mut self, memory: &mut Memory) -> u16 {
        let addr = memory.read_addr(self.reg.pc).wrapping_add(self.reg.x_idx) as u16;
        self.reg.pc += 1;

        let lo = memory.read_addr(addr) as u16;
        let hi = memory.read_addr(addr + 1) as u16;

        lo | hi << 8
    }

    /// Indirect indirect addressing is the most common indirection mode used on the 6502. In
    /// instruction contains the zero page location of the least significant byte of 16 bit
    /// address. The Y register is dynamically added to this value to generated the actual target
    /// address for operation
    ///
    /// Also seen in spec sheets as `Indirect,Y`.
    fn indirect_idx(&mut self, memory: &mut Memory) -> u16 {
        let addr = memory.read_addr(self.reg.pc) as u16;
        self.reg.pc += 1;

        let lo = memory.read_addr(addr) as u16;
        let hi = memory.read_addr(addr + 1) as u16;

        (lo | hi << 8).wrapping_add(self.reg.y_idx as u16)
    }

    /// Execute the opcode and return the number of cycles.
    pub fn execute(&mut self, opcode: u8, memory: &mut Memory) {
        self.reg.pc += 1;

        match opcode {
            0x4c => self.jmp_absolute(memory),
            0x6c => self.jmp_indirect(memory),

            0xa9 => self.lda_immediate(memory),
            0xa5 => self.lda_zero_page(memory),
            0xb5 => self.lda_zero_page_x(memory),
            0xad => self.lda_absolute(memory),
            0xbd => self.lda_absolute_x(memory),
            0xb9 => self.lda_absolute_y(memory),
            0xa1 => self.lda_indirect_x(memory),
            0xb1 => self.lda_indirect_y(memory),

            _ => unimplemented!(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cpu::register::Registers;

    #[test]
    fn immediate_address() {
        let mut memory = Memory::with_bytes(vec![0x00, 0xff]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.pc = 1;

        let addr = cpu.immediate_addr(&mut memory);
        assert_eq!(addr, 0xff);
        assert_eq!(cpu.reg.pc, 2);
    }

    #[test]
    fn zero_page_address() {
        let mut memory = Memory::with_bytes(vec![0x97]);
        let mut cpu = Core::new(Registers::empty());

        let addr = cpu.zero_page_addr(&mut memory);
        assert_eq!(addr, 0x0097);
        assert_eq!(cpu.reg.pc, 1);
    }

    #[test]
    fn zero_page_address_x() {
        let mut memory = Memory::with_bytes(vec![0x97]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.x_idx = 1;

        let addr = cpu.zero_page_addr_x(&mut memory);
        assert_eq!(addr, 0x0098);
        assert_eq!(cpu.reg.pc, 1);
    }

    #[test]
    fn zero_page_address_x_overflow() {
        let mut memory = Memory::with_bytes(vec![0xff]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.x_idx = 3;

        let addr = cpu.zero_page_addr_x(&mut memory);
        assert_eq!(addr, 0x0002);
        assert_eq!(cpu.reg.pc, 1);
    }

    #[test]
    fn zero_page_address_y() {
        let mut memory = Memory::with_bytes(vec![0x97]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.y_idx = 1;

        let addr = cpu.zero_page_addr_y(&mut memory);
        assert_eq!(addr, 0x0098);
        assert_eq!(cpu.reg.pc, 1);
    }

    #[test]
    fn zero_page_address_y_overflow() {
        let mut memory = Memory::with_bytes(vec![0xff]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.y_idx = 3;

        let addr = cpu.zero_page_addr_y(&mut memory);
        assert_eq!(addr, 0x0002);
        assert_eq!(cpu.reg.pc, 1);
    }

    #[test]
    fn relative_address() {
        let mut memory = Memory::with_bytes(vec![0x00, 0x00, 0x12]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.pc = 2;

        let addr = cpu.relative_addr(&mut memory);
        assert_eq!(addr, cpu.reg.pc + 0x12);
        assert_eq!(cpu.reg.pc, 4);
    }

    #[test]
    fn relative_address_overflow() {
        let mut memory = Memory::with_bytes(vec![0x00, 0x00, 0x80]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.pc = 2;

        let addr = cpu.relative_addr(&mut memory);
        assert_eq!(addr, 256 - 0x80 + cpu.reg.pc);
        assert_eq!(cpu.reg.pc, 4);
    }

    #[test]
    fn absolute_address() {
        let mut memory = Memory::with_bytes(vec![0x97, 0x55]);
        let mut cpu = Core::new(Registers::empty());

        let addr = cpu.absolute_addr(&mut memory);
        assert_eq!(addr, 0x5597);
        assert_eq!(cpu.reg.pc, 2);
    }

    #[test]
    fn absolute_address_x() {
        let mut memory = Memory::with_bytes(vec![0x97, 0x55]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.x_idx = 2;

        let addr = cpu.absolute_addr_x(&mut memory);
        assert_eq!(addr, 0x5599);
        assert_eq!(cpu.reg.pc, 2);
    }

    #[test]
    fn absolute_address_x_overflow() {
        let mut memory = Memory::with_bytes(vec![0xff, 0xff]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.x_idx = 2;

        let addr = cpu.absolute_addr_x(&mut memory);
        assert_eq!(addr, 0x0001);
        assert_eq!(cpu.reg.pc, 2);
    }

    #[test]
    fn absolute_address_y() {
        let mut memory = Memory::with_bytes(vec![0x97, 0x55]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.y_idx = 2;

        let addr = cpu.absolute_addr_y(&mut memory);
        assert_eq!(addr, 0x5599);
        assert_eq!(cpu.reg.pc, 2);
    }

    #[test]
    fn absolute_address_y_overflow() {
        let mut memory = Memory::with_bytes(vec![0xff, 0xff]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.y_idx = 2;

        let addr = cpu.absolute_addr_y(&mut memory);
        assert_eq!(addr, 0x0001);
        assert_eq!(cpu.reg.pc, 2);
    }

    #[test]
    fn index_indirect() {
        let mut memory = Memory::with_bytes(vec![0x01, 0xff, 0xff, 0x97, 0x55]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.x_idx = 2;

        let addr = cpu.idx_indirect(&mut memory);
        assert_eq!(addr, 0x5597);
        assert_eq!(cpu.reg.pc, 1);
    }

    #[test]
    fn index_indirect_overflow() {
        let mut memory = Memory::with_bytes(vec![0xff, 0x97, 0x55]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.x_idx = 2;

        let addr = cpu.idx_indirect(&mut memory);
        assert_eq!(addr, 0x5597);
        assert_eq!(cpu.reg.pc, 1);
    }

    #[test]
    fn indirect_index() {
        let mut memory = Memory::with_bytes(vec![0x03, 0xff, 0xff, 0x97, 0x55]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.y_idx = 2;

        let addr = cpu.indirect_idx(&mut memory);
        assert_eq!(addr, 0x5599);
        assert_eq!(cpu.reg.pc, 1);
    }

    #[test]
    fn indirect_index_overflow() {
        let mut memory = Memory::with_bytes(vec![0x01, 0xff, 0xff]);
        let mut cpu = Core::new(Registers::empty());
        cpu.reg.y_idx = 2;

        let addr = cpu.indirect_idx(&mut memory);
        assert_eq!(addr, 0x0001);
        assert_eq!(cpu.reg.pc, 1);
    }
}
