use cpu::Core;
use memory::{ReadAddr, WriteAddr};

type ImpliedFunction = &'static Fn(&mut Core);
type AddressFunction = &'static Fn(&mut Core, u16);
type ValueFunction = &'static Fn(&mut Core, u8);

pub enum Function {
  Address(AddressFunction),
  Value(ValueFunction),
}

impl Function {
  pub fn call(&self, core: &mut Core, memory: &ReadAddr, address: u16) {
    match self {
      Function::Value(func) => func(core, memory.read_addr(address)),
      Function::Address(func) => func(core, address),
    }
  }
}

pub enum Operation {
  /// Accumulator (A)
  ///
  /// OPC A
  ///
  /// operand is AC (implied single byte instruction)
  Accumulator(ValueFunction),

  /// Absolute (abs)
  ///
  /// OPC $LLHH
  ///
  /// operand is address $HHLL *
  Absolute(Function),

  /// Absolute, X-indexed (abs,X)
  ///
  /// OPC $LLHH,X
  ///
  /// operand is address; effective address is address incremented by X with carry **
  AbsoluteX(Function),

  /// Absolute, Y-indexed (abs,Y)
  ///
  /// OPC $LLHH,Y
  ///
  /// operand is address; effective address is address incremented by Y with carry **
  AbsoluteY(Function),

  /// Immediate (#)
  ///
  /// OPC #$BB
  ///
  /// operand is byte BB
  Immediate(ValueFunction),

  /// Implied (impl)
  ///
  /// OPC
  ///
  /// operand implied
  Implied(ImpliedFunction),

  /// Indirect (ind)
  ///
  /// OPC ($LLHH)
  ///
  /// operand is address; effective address is contents of word at address: C.w($HHLL)
  Indirect(Function),

  /// X-indexed, indirect (X,ind)
  ///
  /// OPC ($LL,X)
  ///
  /// operand is zeropage address; effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
  IndirectX(Function),

  /// Indirect, Y-indexed (ind,Y)
  ///
  /// OPC ($LL),Y
  ///
  /// operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry: C.w($00LL) + Y
  IndirectY(Function),

  /// Relative (rel)
  ///
  /// OPC $BB
  ///
  /// branch target is PC + signed offset BB ***
  Relative(Function),

  /// Zeropage (zpg)
  ///
  /// OPC $LL
  ///
  /// operand is zeropage address (hi-byte is zero, address = $00LL)
  Zeropage(Function),

  /// Zeropage, X-indexed (zpg,X)
  ///
  /// OPC $LL,X
  ///
  /// operand is zeropage address; effective address is address incremented by X without carry **
  ZeropageX(Function),

  /// Zeropage, Y-indexed (zpg,Y)
  ///
  /// OPC $LL,Y
  ///
  /// operand is zeropage address; effective address is address incremented by Y without carry **
  ZeropageY(Function),
}

impl Operation {
  pub fn call<M: ReadAddr + WriteAddr>(&self, core: &mut Core, memory: &mut M) {
    match self {
      Operation::Accumulator(func) => {
        let acc = core.reg.acc;
        func(core, acc);
      }

      Operation::Absolute(func) => {
        let addr = core.absolute_addr(memory);
        func.call(core, memory, addr);
      }

      Operation::AbsoluteX(func) => {
        let addr = core.absolute_addr_x(memory);
        func.call(core, memory, addr);
      }

      Operation::AbsoluteY(func) => {
        let addr = core.absolute_addr_y(memory);
        func.call(core, memory, addr);
      }

      Operation::Immediate(func) => {
        let operand = core.immediate_addr(memory);
        func(core, operand);
      }

      Operation::Implied(func) => {
        func(core);
      }

      Operation::IndirectX(func) => {
        let addr = core.idx_indirect(memory);
        func.call(core, memory, addr);
      }

      Operation::IndirectY(func) => {
        let addr = core.indirect_idx(memory);
        func.call(core, memory, addr);
      }

      Operation::Relative(func) => {
        let addr = core.relative_addr(memory);
        func.call(core, memory, addr);
      }

      Operation::Zeropage(func) => {
        let addr = core.zero_page_addr(memory);
        func.call(core, memory, addr);
      }

      Operation::ZeropageX(func) => {
        let addr = core.zero_page_addr_x(memory);
        func.call(core, memory, addr);
      }

      Operation::ZeropageY(func) => {
        let addr = core.zero_page_addr_y(memory);
        func.call(core, memory, addr);
      }

      Operation::Indirect(_) => {
        // TODO: Fix this (and figure out where lo_addr comes from...)
        unimplemented!();
      }
    }
  }
}
