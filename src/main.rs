mod chunk;
mod debug;
mod value;
mod vm;

use chunk::OpCode;
use chunk::Chunk;
use value::Value;
use vm::VM;

fn main() {
    println!("Hello, bite!");
    let mut chunk: Chunk = Chunk::new();
    let mut vm: VM = VM::new();

    let mut constant: usize = chunk.add_constant(Value::ValNumber(4.0));
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant as u8, 123);

    constant = chunk.add_constant(Value::ValNumber(6.0));
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OpAdd as u8, 123);

    constant = chunk.add_constant(Value::ValNumber(5.0));
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OpDivide as u8, 123);

    chunk.write(OpCode::OpNegate as u8, 123);

    chunk.write(OpCode::OpReturn as u8, 123);

    debug::disassemble_chunk(&chunk, "test_chunk");

    vm.interpret(&chunk);

}
