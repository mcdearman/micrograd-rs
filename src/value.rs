use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone)]
pub struct Value {
    pub data: f64,
    pub children: HashSet<Value>,
}

impl Value {
    pub fn new(data: f64) -> Value {
        Value {
            data,
            children: HashSet::new(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        Value::new(self.data + other.data)
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        Value::new(self.data - other.data)
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        Value::new(self.data * other.data)
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        Value::new(self.data / other.data)
    }
}
