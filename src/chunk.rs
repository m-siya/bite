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
    pub fn add_constant(&mut self, value: Value) -> Option<u8> 
    {
        let idx = self.constants.push(value);
        u8::try_from(idx).ok()
    }

    pub fn get_constant(&self, index: usize) -> Value {
        self.constants[index]
    }

}



