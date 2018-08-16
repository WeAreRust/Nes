use memory::Memory:

pub struct Core {
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
  pub status: u8,
}

impl Core {
  pub fn cycle(&mut self, &mut Memory) {
    unimplemented!();
  }
}
