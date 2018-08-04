use cpu::instruction::Instruction;
use cpu::operation::{AddressFunction, Function, ImpliedFunction, Operation, ValueFunction};
use std::str::Lines;

struct InstructionDef<T>
where
  T: Into<ImpliedFunction> + Into<ValueFunction> + Into<AddressFunction> + Into<Function>,
{
  docstring: &'static str,
  function: T,
}

impl<T> InstructionDef<T>
where
  T: Into<ImpliedFunction> + Into<ValueFunction> + Into<AddressFunction> + Into<Function>,
{
  pub fn new(function: T, docstring: &'static str) -> Self {
    InstructionDef {
      function,
      docstring,
    }
  }
}

impl<T> IntoIterator for InstructionDef<T>
where
  T: Into<ImpliedFunction> + Into<ValueFunction> + Into<AddressFunction> + Into<Function>,
{
  type Item = Instruction;
  type IntoIter = InstructionDefIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    let mut doc_iter = self.docstring.lines();

    // Burn the header lines
    let mut header = true;
    while header {
      let line = doc_iter
        .next()
        .expect("Reached end of docstring before finding header seperator");
      header = !line.contains("-----");
    }

    InstructionDefIter {
      doc_iter,
      function: self.function,
    }
  }
}

struct InstructionDefIter<T>
where
  T: Into<ImpliedFunction> + Into<ValueFunction> + Into<AddressFunction> + Into<Function>,
{
  doc_iter: Lines<'static>,
  function: T,
}

impl<T> InstructionDefIter<T>
where
  T: Into<ImpliedFunction> + Into<ValueFunction> + Into<AddressFunction> + Into<Function>,
{
  fn tokenize_line(&self, line: &str) -> [&str; 6] {
    let iter = line.split_whitespace();
    let mut line_arr: [&str; 6] = [""; 6];
    for i in 0..6 {
      line_arr[i] = match iter.next() {
        Some(token) => token,
        None => break,
      }
    }
    line_arr
  }

  fn make_instruction(&self, op: Operation, opcode: &str, cycles: &str) -> Instruction {
    println!()
    unimplemented!();
  }

  fn parse_line(&self, line: &str) -> Option<Instruction> {
    let function = &self.function;
    let inst = match self.tokenize_line(line) {
      ["accumulator", name, "A", opcode, bytes, cycles] => {
        self.make_instruction(Operation::Accumulator(function.into()), opcode, cycles)
      }
      ["absolute", name, "oper", opcode, bytes, cycles] => {
        self.make_instruction(Operation::Absolute(function.into()), opcode, cycles)
      }
      ["absolute,X", name, "oper,X", opcode, bytes, cycles] => {
        self.make_instruction(Operation::AbsoluteX(function.into()), opcode, cycles)
      }
      ["absolute,Y", name, "oper,Y", opcode, bytes, cycles] => {
        self.make_instruction(Operation::AbsoluteY(function.into()), opcode, cycles)
      }
      ["immidiate", name, "#oper", opcode, bytes, cycles] => {
        self.make_instruction(Operation::Immediate(function.into()), opcode, cycles)
      }
      ["implied", name, opcode, bytes, cycles, _] => {
        self.make_instruction(Operation::Implied(function.into()), opcode, cycles)
      }
      ["indirect", name, "(oper)", opcode, bytes, cycles] => {
        self.make_instruction(Operation::Indirect(function.into()), opcode, cycles)
      }
      ["(indirect,X)", name, "(oper,X)", opcode, bytes, cycles] => {
        self.make_instruction(Operation::IndirectX(function.into()), opcode, cycles)
      }
      ["(indirect),Y", name, "(oper),Y", opcode, bytes, cycles] => {
        self.make_instruction(Operation::IndirectY(function.into()), opcode, cycles)
      }
      ["relative", name, "oper", opcode, bytes, cycles] => {
        self.make_instruction(Operation::Relative(function.into()), opcode, cycles)
      }
      ["zeropage", name, "oper", opcode, bytes, cycles] => {
        self.make_instruction(Operation::Zeropage(function.into()), opcode, cycles)
      }
      ["zeropage,X", name, "oper,X", opcode, bytes, cycles] => {
        self.make_instruction(Operation::ZeropageX(function.into()), opcode, cycles)
      }
      ["zeropage,Y", name, "oper,Y", opcode, bytes, cycles] => {
        self.make_instruction(Operation::ZeropageY(function.into()), opcode, cycles)
      }
      _ => return None,
    };
    Some(inst)
  }
}

impl<T> Iterator for InstructionDefIter<T>
where
  T: Into<ImpliedFunction> + Into<ValueFunction> + Into<AddressFunction> + Into<Function>,
{
  type Item = Instruction;

  fn next(&mut self) -> Option<Instruction> {
    let line = self.doc_iter.next()?;
    match self.parse_line(line) {
      Some(inst) => Some(inst),
      None => self.next(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn instruction_def() {
    use cpu::Core;

    fn nop(core: &mut Core) {}

    let def = InstructionDef::new(
      &nop,
      "
        TST  Test Operation

                                         N Z C I D V
                                         + + - - - -

        addressing    assembler     opc  bytes  cyles
        ---------------------------------------------
        accumulator   TST A         FF    2     2
        absolute      TST oper      FF    3     4
        absolute,X    TST oper,X    FF    3     4*
        absolute,Y    TST oper,Y    FF    3     4*
        immidiate     TST #oper     FF    2     2
        implied       TST           FF    2     2
        indirect      TST (oper)    FF    2     2
        (indirect,X)  TST (oper,X)  FF    2     6
        (indirect),Y  TST (oper),Y  FF    2     5
        relative      TST oper      FF    2     4**
        zeropage      TST oper      FF    2     3
        zeropage,X    TST oper,X    FF    2     4
        zeropage,Y    TST oper,Y    FF    2     3
    ",
    );

    for instruction in def.into_iter() {
      assert_eq!(instruction.opcode, 0xFF);
    }
  }
}
