use chunk::Chunk;
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




