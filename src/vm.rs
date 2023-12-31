use std::fmt::Binary;

use crate::debug;
use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::compiler::Compiler;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

//to produce formated runtime error messages
// takes string and a variable number of arguments.
macro_rules! run_time_error {
    ($chunk: expr, $ip: expr, $format: expr $(, $($arg:expr), *)?) => {
        {
            eprintln!($format $(, $($arg), *)?);
            let line = $chunk.lines[$ip - 1];
            eprintln!("[line {}] in script", line);
            //self.reset_stack();
        }
    };
}

pub struct VM {
    //chunk: Chunk,
    ip: usize, //indexes into the next instruction in the chunk
    stack: Vec<Value>,
    stack_top: usize, // points to where next Value will go
}

impl VM {
    pub fn new() -> VM {
        VM {ip: 0, stack: Vec::new(), stack_top: 0}
    }

    pub fn reset_stack(&mut self) {
        self.stack = Vec::new();
    }

    //returns the next instruction to which ip points to
    fn read_byte(&mut self, chunk: &Chunk) -> OpCode{
        let instruction: OpCode = chunk.code[self.ip].into();
        self.ip += 1;
        instruction
        
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let index: usize = chunk.code[self.ip] as usize;
        self.ip += 1;
       // chunk.constants[index]
       chunk.get_constant(index)
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
        self.stack_top += 1;
    }

    fn pop(&mut self) -> Value {
        let value: Option<Value> = self.stack.pop();
        self.stack_top -= 1;
        
        match value {
            Some(value) => return value,
            None => panic!("VM stack is empty"),
        }
    }

    fn peek(&self, depth: usize) -> Value {
        self.stack[self.stack.len() - depth - 1]
    }
 
    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk: Chunk = Chunk::new();
        let mut compiler = Compiler::new(&mut chunk);
        
        if compiler.compile(source).is_err() {
            return InterpretResult::CompileError;
        }

        self.ip = 0;
        let result: InterpretResult = self.run(&chunk);
        result
        
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        macro_rules! BINARY_OP {
            ($op:tt) => {
                {
                    let op_r = self.peek(0);
                    let op_l = self.peek(1);
                    
                    match (op_r.is_number(), op_l.is_number()) {
                        (true, true) => {
                            let right_operand: Value= self.pop();
                            let left_operand: Value = self.pop();
                            self.push(Value::from(left_operand $op right_operand));
                        }
                        (_, _) => {
                            run_time_error!(chunk, self.ip, "Error: {}", "Operands must be numbers");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
            }
        }
        
        loop {

            //only for debugging, put under flag later
            print!("          ");
            for slot in self.stack.iter() {
                print!("[ ");
                (slot).print_value();
                print!(" ]");
            }
            println!();

            debug::disassemble_instruction(chunk, self.ip as u8);
            
            // debug code ends  

            let instruction: OpCode = self.read_byte(chunk);

            match instruction {
                OpCode::OpReturn => {
                    print!("Final value on stack when program returns: ");
                    (self.pop()).print_value();
                    println!();

                    return InterpretResult::Ok;
                }

                OpCode::OpConstant => {
                    let constant = self.read_constant(chunk);
                    self.push(constant);
                    //println!("{}", constant);  
                }
    
                //before case OP_RETURN
                OpCode::OpAdd => BINARY_OP!(+),
                OpCode::OpSubtract => BINARY_OP!(-),
                OpCode::OpMultiply => BINARY_OP!(*),
                OpCode::OpDivide => BINARY_OP!(/),
                OpCode::OpNot => {
                    let value = self.peek(0);
                    match value.is_bool() {
                        true => {
                            let top_val = self.pop();
                            self.push(!top_val);
                        }
                        false => {
                            run_time_error!(&chunk, self.ip, "Error: {}", "Operand must be a boolean");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::OpNegate => {
                    let value = self.peek(0);
                    match value.is_number() {
                        true => {
                            let top_val = self.pop();
                            self.push(-top_val);
                        }

                        false => {
                            run_time_error!(&chunk, self.ip, "Error: {}", "Operand must be a number");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::OpNil => self.push(Value::ValNil(())),
                OpCode::OpTrue => self.push(Value::ValBool(true)),
                OpCode::OpFalse => self.push(Value::ValBool(false)),
                OpCode::OpEqual => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Value::ValBool(a == b));
                }
                OpCode::OpGreater => BINARY_OP!(>),
                OpCode::OpLess => BINARY_OP!(<),
            }
        }        
    }
}




