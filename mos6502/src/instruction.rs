type Op<M> = Fn(&mut Core, &mut Memory, M);

pub enum Instruction {
  Accumulator(&'static Op<mode::Accumulator>),
  Absolute(&'static Op<mode::Absolute>),
}

pub impl Instruction {
  fn call(&self) {
    unimplemented!();
  }
}
