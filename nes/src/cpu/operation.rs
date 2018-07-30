use cpu::Core;

type OperationNone = &'static Fn(&mut Core);
type OperationOne = &'static Fn(&mut Core, u8);

pub enum Operation {
    /// Accumulator (A)
    ///
    /// OPC A
    ///
    /// operand is AC (implied single byte instruction)
    Accumulator(OperationOne),

    /// Absolute (abs)
    ///
    /// OPC $LLHH
    ///
    /// operand is address $HHLL *
    Absolute(OperationOne),

    /// Absolute, X-indexed (abs,X)
    ///
    /// OPC $LLHH,X
    ///
    /// operand is address; effective address is address incremented by X with carry **
    AbsoluteX(OperationOne),

    /// Absolute, Y-indexed (abs,Y)
    ///
    /// OPC $LLHH,Y
    ///
    /// operand is address; effective address is address incremented by Y with carry **
    AbsoluteY(OperationOne),

    /// Immediate (#)
    ///
    /// OPC #$BB
    ///
    /// operand is byte BB
    Immediate(OperationOne),

    /// Implied (impl)
    ///
    /// OPC
    ///
    /// operand implied
    Implied(OperationNone),

    /// Indirect (ind)
    ///
    /// OPC ($LLHH)
    ///
    /// operand is address; effective address is contents of word at address: C.w($HHLL)
    Indirect(OperationOne),

    /// X-indexed, indirect (X,ind)
    ///
    /// OPC ($LL,X)
    ///
    /// operand is zeropage address; effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
    IndirectX(OperationOne),

    /// Indirect, Y-indexed (ind,Y)
    ///
    /// OPC ($LL),Y
    ///
    /// operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry: C.w($00LL) + Y
    IndirectY(OperationOne),

    /// Relative (rel)
    ///
    /// OPC $BB
    ///
    /// branch target is PC + signed offset BB ***
    Relative(OperationOne),

    /// Zeropage (zpg)
    ///
    /// OPC $LL
    ///
    /// operand is zeropage address (hi-byte is zero, address = $00LL)
    Zeropage(OperationOne),

    /// Zeropage, X-indexed (zpg,X)
    ///
    /// OPC $LL,X
    ///
    /// operand is zeropage address; effective address is address incremented by X without carry **
    ZeropageX(OperationOne),

    /// Zeropage, Y-indexed (zpg,Y)
    ///
    /// OPC $LL,Y
    ///
    /// operand is zeropage address; effective address is address incremented by Y without carry **
    ZeropageY(OperationOne),
}
