use value::Value;

#[allow(non_camel_case_types)]
pub enum OpCode {
   
    OP_CONSTANT = 0,
    OP_NEGATE = 1,
    OP_RETURN = 2,


}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> u8 {
        code as u8
    }
}

impl From<u8> for OpCode {
    fn from(index: u8) -> OpCode {
        match index {
            0 => OpCode::OP_CONSTANT,
            1 => OpCode::OP_NEGATE,
            2 => OpCode::OP_RETURN,
            _ => !unimplemented!(),
        }
    }
}

// acces the chunk's capacity and count using vector's .capacity() and .len()
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

    pub fn add_constant(&mut self, value: Value) -> usize{
        self.constants.push(value);
        self.constants.len() - 1
    }
}



