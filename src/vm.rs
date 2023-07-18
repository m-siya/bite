//use crate::value;
use crate::debug;

use chunk::Chunk;
use chunk::OpCode;
use value::Value;
//use debug::*;

pub struct VM {
    chunk: Chunk,
    ip: u8,
    stack: Vec<Value>,
    stack_top: usize, // points to where next Value will go
}

#[allow(non_camel_case_types)]
pub enum InterpretResult {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
}


impl VM {
    pub fn new() -> VM {
        VM {chunk: Chunk::new(), ip: 0, stack: Vec::new(), stack_top: 0}
    }

    fn read_byte(&mut self) -> u8{
        self.ip += 1;
        self.ip
    }

    fn read_constant(&mut self) -> &Value {
        &self.chunk.constants[self.read_byte() as usize]
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
        self.chunk = *chunk;
        self.ip = 0;
        
        self.run()
    }

    fn run(&self) -> InterpretResult {
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
                (*slot).print_value();
                print!(" ]");
            }
            println!();

            debug::disassemble_instruction(&self.chunk, self.ip);
            
            // debug code ends  

            let instruction: OpCode = self.read_byte().into();

            match instruction {
                OpCode::OP_RETURN => {
                    (self.pop()).print_value();
                    println!();

                    return InterpretResult::INTERPRET_OK;
                }

                OpCode::OP_CONSTANT => {
                    let constant: &Value = self.read_constant();
                    self.push(*constant);
                    //println!("{}", constant);  
                }
    
                //before case OP_RETURN
                OpCode::OP_ADD => BINARY_OP!(+),
                OpCode::OP_SUBTRACT => BINARY_OP!(-),
                OpCode::OP_MULTIPLY => BINARY_OP!(*),
                OpCode::OP_DIVIDE => BINARY_OP!(/),
                OpCode::OP_NEGATE => self.push(Value::from(-f64::from(self.pop()))),   
            }
        }        
    }



}




