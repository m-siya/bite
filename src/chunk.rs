use value::Value;

#[allow(non_camel_case_types)]
pub enum OpCode {
    OP_RETURN,
    OP_CONSTANT,

}

// impl OpCode {
//     pub fn to_num(&self) -> u8{
//         *self as u8
//     }
// }

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



