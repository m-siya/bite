//use crate::chunk;
//use value::Value;
//use crate::chunk;
use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: u8 = 0;

    while offset < chunk.code.len() as u8{
        offset = disassemble_instruction(chunk, offset);
    }
}


pub fn disassemble_instruction(chunk: &Chunk, offset: u8) -> u8 {
    print!("{} ", offset);

    if offset > 0 && chunk.lines[offset as usize] == chunk.lines[offset as usize - 1] {
        print!("  | ");
    } else {
        print!("{} ", chunk.lines[offset as usize]);
    }

    let instruction = chunk.code[offset as usize];
    let code: OpCode = instruction.into();
    match code {
        OpCode::OpReturn => return simple_instruction("OP_RETURN", offset),
        OpCode::OpConstant => return constant_instruction("OP_CONSTANT", chunk, offset),
        OpCode::OpNegate => return simple_instruction("OP_NEGATE", offset),
        OpCode::OpAdd => return simple_instruction("OP_ADD", offset),
        OpCode::OpSubtract => return simple_instruction("OP_SUBTRACT", offset),
        OpCode::OpMultiply => return simple_instruction("OP_MULTIPLY", offset),
        OpCode::OpDivide => return simple_instruction("OP_DIVIDE", offset),
        OpCode::OpNot => return simple_instruction("OP_NOT", offset),
        OpCode::OpNil => return simple_instruction("OP_NIL", offset),
        OpCode::OpTrue => return simple_instruction("OP_TRUE", offset),
        OpCode::OpFalse => return simple_instruction("OP_FALSE", offset),
        OpCode::OpEqual => return simple_instruction("OP_EQUAL", offset),
        OpCode::OpGreater => return simple_instruction("OP_GREATER", offset),
        OpCode::OpLess => return simple_instruction("OP_LESS", offset),

        // _ => {
        //     println!("Unknown opcode {}", instruction);
        //     return offset + 1
        // },
    }
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: u8) -> u8 {
    let constant = chunk.code[offset as usize + 1];
    print!("{} {} ", name, constant);
    chunk.constants[constant as usize].print_value();
    println!();
    offset + 2

}

fn simple_instruction(name: &str, offset: u8) -> u8 {
    println!("{}", name);
    offset + 1
}



