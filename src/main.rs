mod chunk;
mod debug;

use chunk::OpCode;
use chunk::Chunk;

fn main() {
    println!("Hello, bite!");
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OP_RETURN as u8);
    debug::disassemble_chunk(&chunk, "test_chunk");

}
