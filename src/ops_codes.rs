use crate::addressing_modes::AddrMode;
use std::collections::HashMap;

pub struct OpCode {
    pub addressing_mode: AddrMode,
    pub assembler: &'static str,
    pub opc: u8,
    pub bytes: u8,
    pub cycles: u8,
}

impl OpCode {
    fn new(mode: AddrMode,assembler: &'static str,opc: u8,bytes: u8,cycles: u8) -> Self {
        OpCode {
            addressing_mode: mode,
            assembler: assembler,
            opc: opc,
            bytes: bytes,
            cycles: cycles,
        }
    }
}

lazy_static! {
    pub static ref OpCodesMap: HashMap<u8,OpCode> = {
        let mut map = HashMap::new();
        // ADC
        map.insert(0x69,OpCode::new(AddrMode::Immediate, "ADC", 0x69,2,2));
        map.insert(0x65,OpCode::new(AddrMode::ZeroPage, "ADC", 0x65,2,3));
        map.insert(0x75,OpCode::new(AddrMode::ZeroPageX,"ADC",0x75,2,4));
        map.insert(0x6D,OpCode::new(AddrMode::Absolute,"ADC",0x6D,3,4));
        map.insert(0x7D,OpCode::new(AddrMode::AbsoluteX,"ADC",0x7D,3,4));
        map.insert(0x79,OpCode::new(AddrMode::AbsoluteY,"ADC",0x79,3,4));
        map.insert(0x61,OpCode::new(AddrMode::IndirectX,"ADC",0x61,2,6));
        map.insert(0x71,OpCode::new(AddrMode::IndirectY,"ADC",0x71,2,5));
        // AND
        map.insert(0x29,OpCode::new(AddrMode::Immediate, "AND",0x29,2,2));
        map.insert(0x25,OpCode::new(AddrMode::ZeroPage, "AND",0x25,2,3));
        map.insert(0x35,OpCode::new(AddrMode::ZeroPageX,"AND",0x35,2,4));
        map.insert(0x2D,OpCode::new(AddrMode::Absolute,"AND",0x2D,3,4));
        map.insert(0x3D,OpCode::new(AddrMode::AbsoluteX,"AND",0x3D,3,4));
        map.insert(0x39,OpCode::new(AddrMode::AbsoluteY,"AND",0x39,3,4));
        map.insert(0x21,OpCode::new(AddrMode::IndirectX,"AND",0x21,2,6));
        map.insert(0x31,OpCode::new(AddrMode::IndirectY,"AND",0x31,2,5));
        // ASL
        map.insert(0x0A,OpCode::new(AddrMode::Accumulator, "ASL",0x0A,1,2));
        map.insert(0x06,OpCode::new(AddrMode::ZeroPage, "ASL",0x06,2,5));
        map.insert(0x16,OpCode::new(AddrMode::ZeroPageX,"ASL",0x16,2,6));
        map.insert(0x0E,OpCode::new(AddrMode::Absolute,"ASL",0x0E,3,6));
        map.insert(0x1E,OpCode::new(AddrMode::AbsoluteX,"ASL",0x1E,3,7));
        // BCC
        map.insert(0x90,OpCode::new(AddrMode::Relative,"BCC",0x90,2,2));
        // BCS
        map.insert(0xB0,OpCode::new(AddrMode::Relative,"BCS",0xB0,2,2));
        // BEQ
        map.insert(0xF0,OpCode::new(AddrMode::Relative,"BEQ",0xF0,2,2));
        // BIT
        map.insert(0x24,OpCode::new(AddrMode::ZeroPage,"BIT",0x24,2,3));
        map.insert(0x2C,OpCode::new(AddrMode::Absolute,"BIT",0x2C,3,4));
        // BMI
        map.insert(0x30,OpCode::new(AddrMode::Relative,"BMI",0x30,2,2));
        // BNE
        map.insert(0xD0,OpCode::new(AddrMode::Relative,"BNE",0xD0,2,2));
        // BPL
        map.insert(0x10,OpCode::new(AddrMode::Relative,"BPL",0x10,2,2));
        // BRK
        map.insert(0x00,OpCode::new(AddrMode::Implied,"BRK",0x00,2,2));
        // BVC
        map.insert(0x50,OpCode::new(AddrMode::Relative,"BVC",0x50,2,2));
        // BVS
        map.insert(0x70,OpCode::new(AddrMode::Relative,"BVS",0x70,2,2));
        // CLC
        map.insert(0x18,OpCode::new(AddrMode::Implied,"CLC",0x18,1,2));
        // CLD
        map.insert(0xD8,OpCode::new(AddrMode::Implied,"CLD",0xD8,1,2));
        // CLI
        map.insert(0x58,OpCode::new(AddrMode::Implied,"CLI",0x58,1,2));
        // CLV
        map.insert(0xB8,OpCode::new(AddrMode::Implied,"CLV",0xB8,1,2));
        // CMP
        map.insert(0xC9,OpCode::new(AddrMode::Immediate,"CMP",0xC9,2,2));
        map.insert(0xC5,OpCode::new(AddrMode::ZeroPage,"CMP",0xC5,2,3));
        map.insert(0xD5,OpCode::new(AddrMode::ZeroPageX,"CMP",0xD5,2,4));
        map.insert(0xCD,OpCode::new(AddrMode::Absolute,"CMP",0xCD,3,4));
        map.insert(0xDD,OpCode::new(AddrMode::AbsoluteX,"CMP",0xDD,3,4));
        map.insert(0xD9,OpCode::new(AddrMode::AbsoluteY,"CMP",0xD9,3,4));
        map.insert(0xC1,OpCode::new(AddrMode::IndirectX,"CMP",0xC1,2,6));
        map.insert(0xD1,OpCode::new(AddrMode::IndirectY,"CMP",0xD1,2,5));
        // CPX
        map.insert(0xE0,OpCode::new(AddrMode::Immediate,"CPX",0xE0,2,2));
        map.insert(0xE4,OpCode::new(AddrMode::ZeroPage,"CPX",0xE4,2,3));
        map.insert(0xEC,OpCode::new(AddrMode::Absolute,"CPX",0xEC,3,4));
        // CPY
        map.insert(0xC0,OpCode::new(AddrMode::Immediate,"CPY",0xC0,2,2));
        map.insert(0xC4,OpCode::new(AddrMode::ZeroPage,"CPY",0xC4,2,3));
        map.insert(0xCC,OpCode::new(AddrMode::Absolute,"CPY",0xCC,3,4));
        // DEC
        map.insert(0xC6,OpCode::new(AddrMode::ZeroPage,"DEC",0xC6,2,5));
        map.insert(0xD6,OpCode::new(AddrMode::ZeroPageX,"DEC",0xD6,2,6));
        map.insert(0xCE,OpCode::new(AddrMode::Absolute,"DEC",0xCE,3,6));
        map.insert(0xDE,OpCode::new(AddrMode::AbsoluteX,"DEC",0xDE,3,7));
        // DEX
        map.insert(0xCA,OpCode::new(AddrMode::Implied,"DEX",0xCA,1,2));
        // DEY
        map.insert(0x88,OpCode::new(AddrMode::Implied,"DEY",0x88,1,2));
        // EOR
        map.insert(0x49,OpCode::new(AddrMode::Immediate,"EOR",0x49,2,2));
        map.insert(0x45,OpCode::new(AddrMode::ZeroPage,"EOR",0x45,2,3));
        map.insert(0x55,OpCode::new(AddrMode::ZeroPageX,"EOR",0x55,2,4));
        map.insert(0x4D,OpCode::new(AddrMode::Absolute,"EOR",0x4D,3,4));
        map.insert(0x5D,OpCode::new(AddrMode::AbsoluteX,"EOR",0x5D,3,4));
        map.insert(0x59,OpCode::new(AddrMode::AbsoluteY,"EOR",0x59,3,4));
        map.insert(0x41,OpCode::new(AddrMode::IndirectX,"EOR",0x41,2,6));
        map.insert(0x51,OpCode::new(AddrMode::IndirectY,"EOR",0x51,2,5));
        // INC
        map.insert(0xE6,OpCode::new(AddrMode::ZeroPage,"INC",0xE6,2,5));
        map.insert(0xF6,OpCode::new(AddrMode::ZeroPageX,"INC",0xF6,2,6));
        map.insert(0xEE,OpCode::new(AddrMode::Absolute,"INC",0xEE,3,6));
        map.insert(0xFE,OpCode::new(AddrMode::AbsoluteX,"INC",0xFE,3,7));
        // INX
        map.insert(0xE8,OpCode::new(AddrMode::Implied,"INX",0xE8,1,2));
        // INY
        map.insert(0xC8,OpCode::new(AddrMode::Implied,"INY",0xC8,1,2));
        // JMP
        map.insert(0x4C,OpCode::new(AddrMode::Absolute,"JMP",0x4C,3,3));
        map.insert(0x6C,OpCode::new(AddrMode::Indirect,"JMP",0x6C,3,5));
        // JSR
        map.insert(0x20,OpCode::new(AddrMode::Absolute,"JSR",0x20,3,6));
        // LDA
        map.insert(0xA9,OpCode::new(AddrMode::Immediate,"LDA",0xA9,2,2));
        map.insert(0xA5,OpCode::new(AddrMode::ZeroPage,"LDA",0xA5,2,3));
        map.insert(0xB5,OpCode::new(AddrMode::ZeroPageX,"LDA",0xB5,2,4));
        map.insert(0xAD,OpCode::new(AddrMode::Absolute,"LDA",0xAD,3,4));
        map.insert(0xBD,OpCode::new(AddrMode::AbsoluteX,"LDA",0xBD,3,4));
        map.insert(0xB9,OpCode::new(AddrMode::AbsoluteY,"LDA",0xB9,3,4));
        map.insert(0xA1,OpCode::new(AddrMode::IndirectX,"LDA",0xA1,2,6));
        map.insert(0xB1,OpCode::new(AddrMode::IndirectY,"LDA",0xB1,2,5));
        // LDX
        map.insert(0xA2,OpCode::new(AddrMode::Implied,"LDX",0xA2,2,2));
        map.insert(0xA6,OpCode::new(AddrMode::ZeroPage,"LDX",0xA6,2,3));
        map.insert(0xB6,OpCode::new(AddrMode::ZeroPageY,"LDX",0xB6,2,4));
        map.insert(0xAE,OpCode::new(AddrMode::Absolute,"LDX",0xAE,3,4));
        map.insert(0xBE,OpCode::new(AddrMode::AbsoluteY,"LDX",0xBE,3,4));
        // LDY
        map.insert(0xA0,OpCode::new(AddrMode::Implied,"LDY",0xA2,2,2));
        map.insert(0xA4,OpCode::new(AddrMode::ZeroPage,"LDY",0xA6,2,3));
        map.insert(0xB4,OpCode::new(AddrMode::ZeroPageX,"LDY",0xB6,2,4));
        map.insert(0xAC,OpCode::new(AddrMode::Absolute,"LDY",0xAE,3,4));
        map.insert(0xBC,OpCode::new(AddrMode::AbsoluteX,"LDY",0xBE,3,4));
        // LSR
        map.insert(0x4A,OpCode::new(AddrMode::Accumulator,"LSR",0x4A,1,2));
        map.insert(0x46,OpCode::new(AddrMode::ZeroPage,"LSR",0x46,2,5));
        map.insert(0x56,OpCode::new(AddrMode::ZeroPageX,"LSR",0x56,1,6));
        map.insert(0x4E,OpCode::new(AddrMode::Absolute,"LSR",0x4E,3,2));
        map.insert(0x5E,OpCode::new(AddrMode::AbsoluteX,"LSR",0x5E,3,7));
        // NOP
        map.insert(0xEA,OpCode::new(AddrMode::Implied,"NOP",0xEA,1,2));
        // ORA
        map.insert(0x09,OpCode::new(AddrMode::Immediate,"ORA",0x09,2,3));
        map.insert(0x05,OpCode::new(AddrMode::ZeroPage,"ORA",0x05,2,3));
        map.insert(0x15,OpCode::new(AddrMode::ZeroPageX,"ORA",0x15,2,4));
        map.insert(0x0D,OpCode::new(AddrMode::Absolute,"ORA",0x0D,3,4));
        map.insert(0x1D,OpCode::new(AddrMode::AbsoluteX,"ORA",0x1D,3,4));
        map.insert(0x19,OpCode::new(AddrMode::AbsoluteY,"ORA",0x19,3,4));
        map.insert(0x01,OpCode::new(AddrMode::IndirectX,"ORA",0x01,2,6));
        map.insert(0x11,OpCode::new(AddrMode::IndirectY,"ORA",0x11,2,5));
        // PHA
        map.insert(0x48,OpCode::new(AddrMode::Implied,"PHA",0x48,1,3));
        // PHP
        map.insert(0x08,OpCode::new(AddrMode::Implied,"PHP",0x08,1,3));
        // PLA
        map.insert(0x68,OpCode::new(AddrMode::Implied,"PHP",0x68,1,4));
        // PLP
        map.insert(0x28,OpCode::new(AddrMode::Implied,"PHP",0x28,1,4));
        // ROL
        map.insert(0x2A,OpCode::new(AddrMode::Accumulator,"ROL",0x2A,1,2));
        map.insert(0x26,OpCode::new(AddrMode::ZeroPage,"ROL",0x26,2,5));
        map.insert(0x36,OpCode::new(AddrMode::ZeroPageX,"ROL",0x36,2,6));
        map.insert(0x2E,OpCode::new(AddrMode::Absolute,"ROL",0x2E,3,6));
        map.insert(0x3E,OpCode::new(AddrMode::AbsoluteX,"ROL",0x3E,3,7));
        // ROR
        map.insert(0x6A,OpCode::new(AddrMode::Accumulator,"ROR",0x6A,1,2));
        map.insert(0x66,OpCode::new(AddrMode::ZeroPage,"ROR",0x66,2,5));
        map.insert(0x76,OpCode::new(AddrMode::ZeroPageX,"ROR",0x76,2,6));
        map.insert(0x6E,OpCode::new(AddrMode::Absolute,"ROR",0x6E,3,6));
        map.insert(0x7E,OpCode::new(AddrMode::AbsoluteX,"ROR",0x7E,3,7));
        // RTI
        map.insert(0x40,OpCode::new(AddrMode::Implied,"RTI",0x40,1,6));
        // RTS
        map.insert(0x60,OpCode::new(AddrMode::Implied,"RTS",0x60,1,6));
        // SBC
        map.insert(0xE9,OpCode::new(AddrMode::Immediate,"SBC",0xE9,2,2));
        map.insert(0xE5,OpCode::new(AddrMode::ZeroPage,"SBC",0xE5,2,3));
        map.insert(0xF5,OpCode::new(AddrMode::ZeroPageX,"SBC",0xF5,2,4));
        map.insert(0xED,OpCode::new(AddrMode::Absolute,"SBC",0xED,3,4));
        map.insert(0xFD,OpCode::new(AddrMode::AbsoluteX,"SBC",0xFD,3,4));
        map.insert(0xF9,OpCode::new(AddrMode::AbsoluteY,"SBC",0xF9,3,4));
        map.insert(0xE1,OpCode::new(AddrMode::IndirectX,"SBC",0xE1,2,6));
        map.insert(0xF1,OpCode::new(AddrMode::IndirectY,"SBC",0xF1,2,5));
        // SEC
        map.insert(0x38,OpCode::new(AddrMode::Implied,"SEC",0x38,1,2));
        // SED
        map.insert(0xF8,OpCode::new(AddrMode::Implied,"SED",0xF8,1,2));
        // SEI
        map.insert(0x78,OpCode::new(AddrMode::Implied,"SEI",0x78,1,2));
        // STA
        map.insert(0x85,OpCode::new(AddrMode::ZeroPage,"STA",0x85,2,3));
        map.insert(0x95,OpCode::new(AddrMode::ZeroPageX,"STA",0x95,2,4));
        map.insert(0x80,OpCode::new(AddrMode::Absolute,"STA",0x80,3,4));
        map.insert(0x90,OpCode::new(AddrMode::AbsoluteX,"STA",0x90,3,5));
        map.insert(0x99,OpCode::new(AddrMode::AbsoluteY,"STA",0x99,3,5));
        map.insert(0x81,OpCode::new(AddrMode::IndirectX,"STA",0x81,2,6));
        map.insert(0x91,OpCode::new(AddrMode::IndirectY,"STA",0x91,2,6));
        // STX
        map.insert(0x86,OpCode::new(AddrMode::ZeroPage,"STX",0x86,2,3));
        map.insert(0x96,OpCode::new(AddrMode::ZeroPageY,"STX",0x96,2,4));
        map.insert(0x8E,OpCode::new(AddrMode::Absolute,"STX",0x8E,3,4));
        // STY
        map.insert(0x84,OpCode::new(AddrMode::ZeroPage,"STY",0x84,2,3));
        map.insert(0x94,OpCode::new(AddrMode::ZeroPageX,"STY",0x94,2,4));
        map.insert(0x8C,OpCode::new(AddrMode::Absolute,"STY",0x8C,3,4));
        // TAX
        map.insert(0xAA,OpCode::new(AddrMode::Implied,"TAX",0xAA,1,2));
        // TAY
        map.insert(0xA8,OpCode::new(AddrMode::Implied,"TAY",0xA8,1,2));
        // TSX
        map.insert(0xBA,OpCode::new(AddrMode::Implied,"TSX",0xBA,1,2));
        // TSA
        map.insert(0x8A,OpCode::new(AddrMode::Implied,"TSA",0x8A,1,2));
        // TXS
        map.insert(0x9A,OpCode::new(AddrMode::Implied,"TXS",0x9A,1,2));
        // TYA
        map.insert(0x98,OpCode::new(AddrMode::Implied,"TYA",0x98,1,2));

        map
    };
}
