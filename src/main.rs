mod chunk;
mod debug;
mod value;
mod vm;
mod compiler;
mod scanner;


use vm::InterpretResult;
use vm::VM;

use std::env;
use std::io;


fn main() {
    println!("Hello, bite!");
    //let mut chunk: Chunk = Chunk::new();
    
    let mut vm: VM = VM::new();

    // let mut constant: usize = chunk.add_constant(Value::ValNumber(4.0));
    // chunk.write(OpCode::OpConstant as u8, 123);
    // chunk.write(constant as u8, 123);

    // constant = chunk.add_constant(Value::ValNumber(6.0));
    // chunk.write(OpCode::OpConstant as u8, 123);
    // chunk.write(constant as u8, 123);

    // chunk.write(OpCode::OpAdd as u8, 123);

    // constant = chunk.add_constant(Value::ValNumber(5.0));
    // chunk.write(OpCode::OpConstant as u8, 123);
    // chunk.write(constant as u8, 123);

    // chunk.write(OpCode::OpDivide as u8, 123);

    // chunk.write(OpCode::OpNegate as u8, 123);

    // chunk.write(OpCode::OpReturn as u8, 123);

    // debug::disassemble_chunk(&chunk, "test_chunk");

    // vm.interpret(&chunk);

    let args: Vec<String> = env::args().collect();
    let argc = args.len() - 1;

    match argc {
        1 => {
            repl(&mut vm);
        },
        2 => {
            run_file(&vm, &args[1]);
        },
        _ => {
            println!("Incorrect arguments.");
            println!("Usage: bite [path]");
            std::process::exit(64);
        },
    }
}

fn repl(vm: &mut VM) {
    loop {
        print!("> ");

        let mut line = String::new();

        match io::stdin().read_line(&mut line).unwrap() {
            0 => {
                println!();
                break;
            },
            _ => {
                vm.interpret(&line);
            },
        }

    }
}

fn run_file(vm: &VM, path: &str) -> io::Result<()> {
    let buffer = std::fs::read_to_string(path)?;
    match vm.interpret(&buffer) {
        InterpretResult::CompileError => std::process::exit(65),
        InterpretResult::RuntimeError => std::process::exit(70),
        InterpretResult::Ok => std::process::exit(0),
    }
}