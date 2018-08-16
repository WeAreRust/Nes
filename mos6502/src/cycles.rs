pub enum Cycles {
  /// constant cycles
  C(usize),

  /// add 1 to cycles if page boundery is crossed
  PB(usize),

  /// add 1 to cycles if branch occurs on same page
  /// add 2 to cycles if branch occurs to different page
  BR(usize),
}
