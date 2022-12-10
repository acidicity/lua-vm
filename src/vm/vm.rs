use crate::instruction::OpCode;

pub struct VM {
    /// Registers are blocks which store temporary values
    pub registers: [i32; 32],
    /// Program counter which tracks the current byte in the program
    pub pc: usize,
    /// The bytecode of the program
    pub program: Vec<u8>,
    /// Contains the result of the modulo operation
    remainder: u32,
    /// Contains the result of the last comparison operation
    eq_flag: bool,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
            program: Vec::new(),
            remainder: 0,
            eq_flag: false,
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                break;
            }

            match self.decode_opcode() {
                OpCode::LOAD => {
                    let register = self.next_8_bits() as usize;
                    let number = self.next_8_bits() as u16;
                    self.registers[register] = number as i32;
                    continue;
                }

                OpCode::JMP => {
                    let target = self.registers[self.next_8_bits() as usize];
                    self.pc = target as usize;
                }

                OpCode::JMPF => {
                    let value = self.registers[self.next_8_bits() as usize];
                    self.pc += value as usize;
                }

                OpCode::JMPB => {
                    let value = self.registers[self.next_8_bits() as usize];
                    self.pc -= value as usize;
                }

                OpCode::ADD => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 + register2;
                }

                OpCode::SUB => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 - register2;
                }

                OpCode::MUL => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 * register2;
                }

                OpCode::DIV => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.registers[self.next_8_bits() as usize] = register1 / register2;
                    self.remainder = (register1 % register2) as u32;
                }

                OpCode::EQ => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.eq_flag = register1 == register2;
                    self.next_8_bits();
                }

                OpCode::NEQ => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.eq_flag = register1 != register2;
                    self.next_8_bits();
                }
                OpCode::GT => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.eq_flag = register1 > register2;
                    self.next_8_bits();
                }
                OpCode::LT => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.eq_flag = register1 < register2;
                    self.next_8_bits();
                }
                OpCode::GTE => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.eq_flag = register1 >= register2;
                    self.next_8_bits();
                }
                OpCode::LTE => {
                    let register1 = self.registers[self.next_8_bits() as usize];
                    let register2 = self.registers[self.next_8_bits() as usize];
                    self.eq_flag = register1 <= register2;
                    self.next_8_bits();
                }
                OpCode::JMPE => {
                    if self.eq_flag {
                        let register = self.next_8_bits() as usize;
                        let target = self.registers[register];
                        self.pc = target as usize;
                    }
                }

                OpCode::HLT => {
                    println!("HLT encountered");
                    return;
                }
                _ => {
                    println!("Unrecognized opcode found!");
                    return;
                }
            }
        }
    }

    pub fn add_bytes(&mut self, program: Vec<u8>) {
        self.program = program;
        println!("{:?}", self.program)
    }

    fn decode_opcode(&mut self) -> OpCode {
        let opcode = OpCode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }
}
