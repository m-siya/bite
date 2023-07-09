//use crate::chunk;
use value::Value;
use chunk::Chunk;
use chunk::OpCode;


pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: u8 = 0;

    while offset < chunk.code.len() as u8{
        offset = disassemble_instruction(chunk, offset);
    }
}


fn disassemble_instruction(chunk: &Chunk, offset: u8) -> u8 {
    print!("{} ", offset);

    if offset > 0 && chunk.lines[offset as usize] == chunk.lines[offset as usize - 1] {
        print!("  | ");
    } else {
        print!("{} ", chunk.lines[offset as usize]);
    }

    let instruction = chunk.code[offset as usize];
    match instruction {
        opcode if opcode == OpCode::OP_RETURN as u8 => return simple_instruction("OP_RETURN", offset),
        opcode if opcode == OpCode::OP_CONSTANT as u8 => return constant_instruction("OP_CONSTANT", chunk, offset),
        _ => {
            println!("Unknown opcode {}", instruction);
            return offset + 1
        },
    }
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: u8) -> u8 {
    let constant = chunk.code[offset as usize + 1];
    print!("{} {} ", name, constant);
    chunk.constants[constant as usize].print_value();
    offset + 2

}

fn simple_instruction(name: &str, offset: u8) -> u8 {
    println!("{}", name);
    offset + 1
}



