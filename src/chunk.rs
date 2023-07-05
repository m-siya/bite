#[allow(non_camel_case_types)]
pub enum OpCode {
    OP_RETURN,
}

// impl OpCode {
//     pub fn to_num(&self) -> u8{
//         *self as u8
//     }
// }

// acces the chunk's capacity and count using vector's .capacity() and .len()
pub struct Chunk {
    pub code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Chunk{
        Chunk {code: Vec::new()}
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }
}



