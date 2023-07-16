use chunk::Chunk;
use chunk::OpCode;
use value::Value;


pub struct VM {
    chunk: Chunk,
    ip: u8,
    stack: Vec<Value>,
    stack_top: usize, // points to where next Value will go
}

impl VM {
    pub fn new() -> VM {
        //TO DO
    }

    pub fn free() {
        //TO DO
    }

    fn run() -> InterpretResult {
        //TO DO

        //after #ifdef DEBUG_TRACE_EXECUTION
        print!("          ");
        for slot in vm.stack.iter() {
            print!("[ ");
            printValue(*slot);
            print!(" ]");
        }
        println!();

        //after #define READ_CONSTANT() (vm.chunk->constants.values[READ_BYTE()])
        macro_rules! BINARY_OP {
            ($op:tt) => {
                {
                    let b = pop();
                    let a = pop();
                    push(a $op b);
                }
            }
        }

        //after #undef BINARY_OP
        


        match code {
            //TO DO

            //under case OP_CONSTANT.. Value constant = READ_CONSTANT();
            push(constant);
            //break;
            
            OpCode::OP_RETURN => {
                printValue(pop());
                println!();
            }

            //before case OP_RETURN
            OpCode::OP_ADD => BINARY_OP(+);
            OpCode::OP_SUBTRACT => BINARY_OP(-);
            OpCode::OP_MULTIPLY => BINARY_OP(*);
            OpCode::OP_DIVIDE => BINARY_OP(/);
            OpCode::OP_NEGATE => push(-pop());
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push_back(value);
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> Value {
        let value: Value = self.stack.pop();
        self.stack_top -= 1
        value
    }


}




