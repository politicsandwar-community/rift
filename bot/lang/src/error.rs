use crate::{ast::Op, Value};

#[derive(Debug)]
pub enum CompileError {
    ParseError(String),
    ParseTimeError(time::error::Parse, String),
    ParseIntError(std::num::ParseIntError, String),
    ParseFloatError(std::num::ParseFloatError, String),
    ParseDecimalError(bigdecimal::ParseBigDecimalError, String),
}

#[derive(Clone, Debug)]
pub enum RuntimeError {
    VariableNotFound(String),
    AttributeNotFound(String),
    StaticAttributeNotFound(String),
    NotIndexable(Value),
    IndexOutOfBounds(i64),
    NotCallable(Value),
    InvalidOp(Op, Option<Value>, Value),
    DivisionByZero,
}

#[derive(Clone, Debug)]
pub enum TypeError {}
