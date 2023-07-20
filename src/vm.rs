//use crate::value;
use crate::debug;

use chunk::Chunk;
use chunk::OpCode;
use value::Value;
//use debug::*;
pub enum InterpretResult {
    Ok,
    //CompileError,
   // RuntimeError,
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

    // pub fn reset_stack(&mut self) {
    //     self.stack = Vec::new();
    // }

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

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {  
        self.ip = 0;
        self.run(chunk)
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        macro_rules! BINARY_OP {
            ($op:tt) => {
                {
                    let right_operand: f64 = self.pop().into();
                    let left_operand: f64 = self.pop().into();
                    self.push(Value::from(left_operand $op right_operand));
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
                OpCode::OpNegate => {
                    let value = self.pop();
                    self.push(-value)
                }
            }
        }        
    }



}




