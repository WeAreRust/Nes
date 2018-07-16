#[derive(Debug, PartialEq)]
pub struct Pipeline {
    /// Next instruction to execute
    pub opcode: Option<u8>,

    /// Cycles remaining until `next` is executed
    pub rem_cycles: usize,
}

impl Default for Pipeline {
    fn default() -> Self {
        Pipeline::new(None, 0)
    }
}

impl Iterator for Pipeline {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem_cycles > 1 {
            self.rem_cycles -= 1;
            return None;
        }

        let opcode = self.opcode;
        self.opcode = None;
        self.rem_cycles = 0;
        opcode
    }
}

impl Pipeline {
    pub fn new(opcode: Option<u8>, rem_cycles: usize) -> Self {
        Pipeline { opcode, rem_cycles }
    }

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
        let pipeline = Pipeline::default();

        assert!(!pipeline.has_next());
    }

    #[test]
    fn set_next_instruction() {
        let mut pipeline = Pipeline::default();
        pipeline.set_next(0xff, 2);

        assert_eq!(pipeline.opcode, Some(0xff));
        assert_eq!(pipeline.rem_cycles, 2);
    }

    #[test]
    fn default_iteration() {
        let mut pipeline = Pipeline::default();

        assert_eq!(pipeline.next(), None);
        assert_eq!(pipeline.rem_cycles, 0);
    }

    #[test]
    fn initial_iteration() {
        let mut pipeline = Pipeline::default();
        pipeline.set_next(0xff, 2);

        assert_eq!(pipeline.rem_cycles, 2);
        assert_eq!(pipeline.next(), None);

        assert_eq!(pipeline.rem_cycles, 1);
        assert_eq!(pipeline.next(), Some(0xff));
    }

    #[test]
    fn overflow_iteration() {
        let mut pipeline = Pipeline::default();
        pipeline.set_next(0xff, 1);

        assert_eq!(pipeline.next(), Some(0xff));
        assert_eq!(pipeline.rem_cycles, 0);

        assert_eq!(pipeline.next(), None);
        assert_eq!(pipeline.rem_cycles, 0);
    }
}
