mod chunk;
mod debug;
mod value;
mod vm;

use chunk::OpCode;
use chunk::Chunk;
use value::Value;
use vm::VM;
use vm::InterpretResult;

fn main() {
    println!("Hello, bite!");
    let mut chunk: Chunk = Chunk::new();
    let mut vm: VM = VM::new();

    let mut constant = chunk.add_constant(Value::VAL_NUMBER(1.2));
    chunk.write(OpCode::OP_CONSTANT as u8, 123);
    chunk.write(constant as u8, 123);

    constant = chunk.add_constant(Value::VAL_NUMBER(3.4));
    chunk.write(OpCode::OP_CONSTANT as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OP_ADD as u8, 123);

    constant = chunk.add_constant(Value::VAL_NUMBER(5.6));
    chunk.write(OpCode::OP_CONSTANT as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OP_DIVIDE as u8, 123);

    chunk.write(OpCode::OP_NEGATE as u8, 123);

    chunk.write(OpCode::OP_RETURN as u8, 123);

    debug::disassemble_chunk(&chunk, "test_chunk");

    let result: vm::InterpretResult = vm.interpret(&chunk);

    
   

  

}
