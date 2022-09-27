use crate::{ast::Op, Value};

#[derive(Clone, Debug)]
pub enum CompileError {
    NumberOutOfRange,
    ParseError(String),
}

#[derive(Clone, Debug)]
pub enum RuntimeError {
    VariableNotFound(String),
    AttributeNotFound(String),
    NotIndexable(Value),
    IndexOutOfBounds(i64),
    NotCallable(Value),
    InvalidOp(Op, Option<Value>, Value),
    DivisionByZero,
}

#[derive(Clone, Debug)]
pub enum TypeError {}
