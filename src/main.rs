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

    constant = chunk.add_constant(Value::Number(3.4));
    chunk.write(OpCode::OP_CONSTANT as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OP_ADD as u8, 123);

    constant = chunk.add_constant(Value::Number(5.6));
    chunk.write(OpCode::OP_CONSTANT as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OP_DIVIDE as u8, 123);
    chunk.write(OpCode::OP_NEGATE as u8, 123);

    chunk.write(OpCode::OP_RETURN as u8, 123);
    debug::disassemble_chunk(&chunk, "test_chunk");

}
