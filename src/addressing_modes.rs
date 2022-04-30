/// Accumulator,OPC A	operand is AC (implied single byte instruction)
/// absolute	OPC $LLHH	operand is address $HHLL *
/// X	absolute, X-indexed	OPC $LLHH,X	operand is address; effective address is address incremented by X with carry **
/// Y	absolute, Y-indexed	OPC $LLHH,Y	operand is address; effective address is address incremented by Y with carry **
/// immediate	OPC #$BB	operand is byte BB
/// implied	OPC	operand implied
/// indirect	OPC ($LLHH)	operand is address; effective address is contents of word at address: C.w($HHLL)
/// X-indexed, indirect	OPC ($LL,X)	operand is zeropage address; effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
/// indirect, Y-indexed	OPC ($LL),Y	operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry: C.w($00LL) + Y
/// relative	OPC $BB	branch target is PC + signed offset BB ***
/// zeropage	OPC $LL	operand is zeropage address (hi-byte is zero, address = $00LL)
/// zeropage, X-indexed	OPC $LL,X	operand is zeropage address; effective address is address incremented by X without carry **
/// zeropage, Y-indexed	OPC $LL,Y	operand is zeropage address; effective address is address incremented by Y without carry **

pub enum AddrMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}