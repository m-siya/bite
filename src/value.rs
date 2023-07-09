pub enum Value {
    Number(f64),
}

impl Value {
    pub fn print_value(&self) {
        match *self {
            Value::Number(val) => println!("'{}'", val),
            _ => println!("Error"),
        }
    }
}


