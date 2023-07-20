use std::ops::Neg;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Value {
    VAL_BOOL(bool),
    VAL_NIL(()),
    VAL_NUMBER(f64),
}

impl From<Value> for bool {
    fn from(value: Value) -> bool {
        match value{
            Value::VAL_BOOL(bool_val) => bool_val,
            _ => panic!("Error. Value is not boolean"),
        }
    }
}

impl From<Value> for f64 {
    fn from(value: Value) -> f64 {
        match value {
            Value::VAL_NUMBER(num_val) => num_val,
            _ => panic!("Error. Value is not numeric"),
        }
    }
}

impl From<Value> for () {
    fn from(value: Value) -> () {
        match value {
            Value::VAL_NIL(nil_val) => nil_val,
            _ => panic!("Error. Value is not nill"),
        }
    }
}

impl From<bool> for Value {
    fn from(bool_val: bool) -> Value {
        Value::VAL_BOOL(bool_val)
    }
}

impl From<f64> for Value {
    fn from(num_val: f64) -> Value {
        Value::VAL_NUMBER(num_val)
    }
}

impl From<()> for Value {
    fn from(nil_val: ()) -> Value {
        Value::VAL_NIL(nil_val)
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::VAL_NUMBER(num) => Value::VAL_NUMBER(-num),
            _ => panic!("Error. Negating a non numeric value is not possible"),
        }
    }
}

impl Value {
    pub fn print_value(&self) {
        match *self {
            Value::VAL_NUMBER(val) => println!("'{}'", val),
            _ => panic!("Value not recognised, cannot print"),
        }
    }

   // pub fn read_value(&self, which: usize)
}


