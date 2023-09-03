use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub data: f64,
    pub children: Vec<Value>,
    pub op: Option<Op>,
}

impl Value {
    pub fn new(data: f64) -> Value {
        Value {
            data,
            children: vec![],
            op: None,
        }
    }

    fn new_with_children(data: f64, children: Vec<Value>, op: Option<Op>) -> Value {
        Value { data, children, op }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value({})", self.data)
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        Value::new_with_children(
            self.data.clone() + other.data.clone(),
            vec![self.clone(), other.clone()],
            Some(Op::Add),
        )
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Value) -> Value {
        Value::new_with_children(
            self.data - other.data,
            vec![self.clone(), other.clone()],
            Some(Op::Sub),
        )
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        Value::new_with_children(
            self.data * other.data,
            vec![self.clone(), other.clone()],
            Some(Op::Mul),
        )
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Value) -> Value {
        Value::new_with_children(
            self.data / other.data,
            vec![self.clone(), other.clone()],
            Some(Op::Div),
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

// Builds a set of all nodes and edges in a graph
pub fn trace(root: &Value) -> (Vec<Value>, Vec<(Value, Value)>) {
    let mut nodes: Vec<Value> = vec![];
    let mut edges: Vec<(Value, Value)> = vec![];
    // fn build(v: &Value)
    (nodes, edges)
}
