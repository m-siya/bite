use std::ops::Neg;

#[derive(Copy, Clone)]
pub enum Value {
    ValBool(bool),
    ValNil(()),
    ValNumber(f64),
}

impl From<Value> for bool {
    fn from(value: Value) -> bool {
        match value{
            Value::ValBool(bool_val) => bool_val,
            _ => panic!("Error. Value is not boolean"),
        }
    }
}

impl From<Value> for f64 {
    fn from(value: Value) -> f64 {
        match value {
            Value::ValNumber(num_val) => num_val,
            _ => panic!("Error. Value is not numeric"),
        }
    }
}

impl From<Value> for () {
    fn from(value: Value) -> () {
        match value {
            Value::ValNil(nil_val) => nil_val,
            _ => panic!("Error. Value is not nill"),
        }
    }
}

impl From<bool> for Value {
    fn from(bool_val: bool) -> Value {
        Value::ValBool(bool_val)
    }
}

impl From<f64> for Value {
    fn from(num_val: f64) -> Value {
        Value::ValNumber(num_val)
    }
}

impl From<()> for Value {
    fn from(nil_val: ()) -> Value {
        Value::ValNil(nil_val)
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::ValNumber(num) => Value::ValNumber(-num),
            _ => panic!("Error. Negating a non numeric value is not possible"),
        }
    }
}

impl Value {
    pub fn print_value(&self) {
        match *self {
            Value::ValNumber(val) => print!("'{}'", val),
            _ => panic!("Value not recognised, cannot print"),
        }
    }

   // pub fn read_value(&self, which: usize)
}


