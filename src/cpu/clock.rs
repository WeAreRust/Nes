pub struct Clock {
    /// Next instruction to execute
    pub opcode: Option<u8>,

    /// Cycles remaining until `next` is executed
    pub rem_cycles: usize,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            opcode: None,
            rem_cycles: 0,
        }
    }
}

impl Iterator for Clock {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem_cycles > 0 {
            self.rem_cycles -= 1;
            return None;
        }
        self.opcode
    }
}

impl Clock {
    pub fn has_next(&self) -> bool {
        self.opcode.is_some()
    }

    pub fn set_next(&mut self, opcode: u8, cycles: usize) {
        self.opcode = Some(opcode);
        self.rem_cycles = cycles;
    }
}
