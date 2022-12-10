#[derive(Debug, PartialEq)]
pub enum OpCode {
    LOAD,
    JMP,
    JMPF,
    JMPB,
    ADD,
    SUB,
    MUL,
    DIV,
    EQ,
    NEQ,
    GT,
    LT,
    GTE,
    LTE,
    JMPE,
    HLT,
    IGL,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: OpCode,
}

impl Instruction {
    fn new(opcode: OpCode) -> Self {
        Self { opcode }
    }
}

impl From<u8> for OpCode {
    fn from(v: u8) -> Self {
        match v {
            0 => OpCode::HLT,
            1 => OpCode::LOAD,
            2 => OpCode::ADD,
            3 => OpCode::SUB,
            4 => OpCode::MUL,
            5 => OpCode::DIV,
            6 => OpCode::JMP,
            7 => OpCode::JMPF,
            8 => OpCode::JMPB,
            9 => OpCode::EQ,
            10 => OpCode::NEQ,
            11 => OpCode::GT,
            12 => OpCode::LT,
            13 => OpCode::GTE,
            14 => OpCode::LTE,
            15 => OpCode::JMPE,
            _ => OpCode::IGL,
        }
    }
}
