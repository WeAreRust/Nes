/// CPU registers, including registers specific to the ALU
///
/// The registers on the NES CPU are just like on the 6502. There is the accumulator, 2 indexes, a
/// program counter, the stack pointer, and the status register. Unlike many CPU families, members
/// do not have generic groups of registers like say, R0 through R7.
pub struct Registers {
  /// Accumulator register (A)
  pub acc: u8,

  /// Index register (X)
  ///
  /// It can be set to a value retrieved from memory and can be used to get or set the value of
  /// the stack pointer.
  pub x_idx: u8,

  /// Index register (Y)
  ///
  /// It can be set to a value retrieved from memory but cannot be used to get or set the value
  /// of the stack pointer.
  pub y_idx: u8,

  /// Program counter (PC)
  pub pc: u16,

  /// Stack pointer (SP)
  pub stack: u8,

  /// Status register (P)
  pub status: StatusFlags,
}

impl Registers {
  pub fn empty() -> Self {
    Registers {
      acc: 0,
      x_idx: 0,
      y_idx: 0,
      pc: 0,
      stack: 0,
      status: StatusFlags::empty(),
    }
  }
}

impl Default for Registers {
  fn default() -> Self {
    Registers {
      acc: 0,
      x_idx: 0,
      y_idx: 0,
      pc: 0xc00,
      stack: 0x24,
      status: StatusFlags::default(),
    }
  }
}

bitflags! {
    /// Status register
    ///
    /// 7 6 5 4 3 2 1 0
    /// N V X B D I Z C
    /// | |   | | | | +--- Carry Flag
    /// | |   | | | +----- Zero Flag
    /// | |   | | +------- Interrupt Disable
    /// | |   | +--------- Decimal Mode (unused)
    /// | |   +----------- Break Command
    /// | +--------------- Overflow Flag
    /// +----------------- Negative Flag
    pub struct StatusFlags: u8 {
       const C_FLAG = 0b0000_0001;
       const Z_FLAG = 0b0000_0010;
       const I_FLAG = 0b0000_0100;
       const D_FLAG = 0b0000_1000; //unused, always on
       const B_FLAG = 0b0001_0000;
       const X_FLAG = 0b0010_0000; //unused, always on
       const V_FLAG = 0b0100_0000;
       const N_FLAG = 0b1000_0000;

       const NZ_FLAG = Self::N_FLAG.bits | Self::Z_FLAG.bits;
       const NZC_FLAG = Self::NZ_FLAG.bits | Self::C_FLAG.bits;
       const NVZC_FLAG = Self::NZC_FLAG.bits | Self::V_FLAG.bits;
       const NV_FLAG = Self::N_FLAG.bits | Self::V_FLAG.bits;
       const DX_FLAG = Self::D_FLAG.bits | Self::X_FLAG.bits;
    }
}

impl StatusFlags {
  /// Carry flag is... 9th bit of u16...
  pub fn set_carry(&mut self, result: u16) {
    self.set(Self::C_FLAG, ((result >> 8) & 1) == 1);
  }

  /// Zero flag is set if the result of the last operation as was zero
  pub fn set_zero(&mut self, result: u8) {
    self.set(Self::Z_FLAG, result == 0)
  }

  /// Overflow flag is... out of range [-128, 127]...
  ///
  /// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html The definition of the
  /// 6502 overflow flag is that it is set if the result of a signed addition or subtraction
  /// doesn't fit into a signed byte. That is, overflow occurs if the result is > 127 or < -128.
  /// The symptom of this is adding two positive numbers and getting a negative result or adding
  /// two negative numbers and getting a positive result.
  pub fn set_overflow(&mut self, result: u16) {
    self.set(Self::V_FLAG, result > 127 || (result as i16) < -128);
  }

  /// Negative flag is set if the result of the last operation had bit 7 set to 1
  ///
  /// TODO: Explain why it's the 7th flag (2's compliment).
  pub fn set_negative(&mut self, result: u8) {
    self.set(Self::N_FLAG, (result >> 7) == 1);
  }
}

impl Default for StatusFlags {
  fn default() -> Self {
    Self::DX_FLAG
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default_status_flags() {
    assert_eq!(StatusFlags::default().bits, 0b00101000);
  }

  #[test]
  fn carry_flag_hi() {
    let mut flags = StatusFlags::empty();
    flags.set_carry(0b0000_0001_1111_1110);

    assert!(flags.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn carry_flag_lo() {
    let mut flags = StatusFlags::empty();
    flags.set_carry(0b0000_0000_1111_1110);

    assert!(!flags.contains(StatusFlags::C_FLAG));
  }

  #[test]
  fn zero_flag_hi() {
    let mut flags = StatusFlags::empty();
    flags.set_zero(0);

    assert!(flags.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn zero_flag_lo() {
    let mut flags = StatusFlags::empty();
    flags.set_zero(1);

    assert!(!flags.contains(StatusFlags::Z_FLAG));
  }

  #[test]
  fn overflow_flag_hi_over() {
    let mut flags = StatusFlags::empty();
    flags.set_overflow(128);

    assert!(flags.contains(StatusFlags::V_FLAG));
  }

  #[test]
  fn overflow_flag_hi_under() {
    let mut flags = StatusFlags::empty();
    flags.set_overflow(-129i16 as u16);

    assert!(flags.contains(StatusFlags::V_FLAG));
  }

  #[test]
  fn overflow_flag_lo() {
    let mut flags = StatusFlags::empty();
    flags.set_overflow(1);

    assert!(!flags.contains(StatusFlags::V_FLAG));
  }

  #[test]
  fn negative_flag_hi() {
    let mut flags = StatusFlags::empty();
    flags.set_negative(0b10011000);

    assert!(flags.contains(StatusFlags::N_FLAG));
  }

  #[test]
  fn negative_flag_lo() {
    let mut flags = StatusFlags::empty();
    flags.set_negative(0);

    assert!(!flags.contains(StatusFlags::N_FLAG));
  }

}
