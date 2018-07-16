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

        let opcode = self.opcode;
        self.opcode = None;
        opcode
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_next_instruction() {
        let clock = Clock::default();

        assert!(!clock.has_next());
    }

    #[test]
    fn set_next_instruction() {
        let mut clock = Clock::default();
        clock.set_next(0xff, 2);

        assert_eq!(clock.opcode, Some(0xff));
        assert_eq!(clock.rem_cycles, 2);
    }

    #[test]
    fn default_iteration() {
        let mut clock = Clock::default();

        assert_eq!(clock.next(), None);
    }

    #[test]
    fn initial_iteration() {
        let mut clock = Clock::default();
        clock.set_next(0xff, 2);

        assert_eq!(clock.next(), None);
        assert_eq!(clock.next(), None);
        assert_eq!(clock.next(), Some(0xff));
    }

    #[test]
    fn overflow_iteration() {
        let mut clock = Clock::default();
        clock.set_next(0xff, 0);

        assert_eq!(clock.next(), Some(0xff));
        assert_eq!(clock.next(), None);
    }
}
