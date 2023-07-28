//use crate::value;
use crate::value::Value;

#[derive(Clone, Copy)]
pub enum OpCode {
   
    OpConstant,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> u8 {
        code as u8
    }
}

impl From<u8> for OpCode {
    fn from(index: u8) -> OpCode {
        match index {
            0 => OpCode::OpConstant,
            1 => OpCode::OpAdd,
            2 => OpCode::OpSubtract,
            3 => OpCode::OpMultiply,
            4 => OpCode::OpDivide,
            5 => OpCode::OpNegate,
            6 => OpCode::OpReturn,
            7 => OpCode::OpNil,
            8 => OpCode::OpTrue,
            9 => OpCode::OpFalse,
            10 => OpCode::OpNot,
            11 => OpCode::OpEqual,
            12 => OpCode::OpGreater,
            13 => OpCode::OpLess,
            _ => panic!("Error. Invalid OpCode code")
        }
    }
}

// access the chunk's capacity and count using vector's .capacity() and .len()
//#[derive(Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Chunk{
        Chunk {code: Vec::new(), constants: Vec::new(), lines: Vec::new()}
    }

    pub fn write(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    // add_constant returns u8 
    pub fn add_constant(&mut self, value: Value) -> usize
    {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> Value {
        self.constants[index]
    }

}



