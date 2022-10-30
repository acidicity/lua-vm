#[derive(Debug, PartialEq)]
pub enum Opcode {
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
    opcode: Opcode,
}

impl Instruction {
    fn new(opcode: Opcode) -> Self {
        Self { opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::LOAD,
            2 => Opcode::ADD,
            3 => Opcode::SUB,
            4 => Opcode::MUL,
            5 => Opcode::DIV,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTE,
            14 => Opcode::LTE,
            15 => Opcode::JMPE,
            _ => Opcode::IGL,
        }
    }
}
