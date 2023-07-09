mod chunk;
mod debug;
mod value;

use chunk::OpCode;
use chunk::Chunk;
use value::Value;

fn main() {
    println!("Hello, bite!");
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(Value::Number(1.2));
    chunk.write(OpCode::OP_CONSTANT as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OP_RETURN as u8, 123);
    debug::disassemble_chunk(&chunk, "test_chunk");

}
