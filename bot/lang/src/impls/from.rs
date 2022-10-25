use bigdecimal::BigDecimal;
use dashmap::DashMap;

use crate::{
    ast::{Arr, Func, Map},
    Ast, Value, Var,
};

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value as i64)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<BigDecimal> for Value {
    fn from(value: BigDecimal) -> Self {
        Value::Decimal(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<DashMap<String, Ast>> for Value {
    fn from(value: DashMap<String, Ast>) -> Self {
        Value::Map(Map(value))
    }
}

impl From<Vec<Ast>> for Value {
    fn from(value: Vec<Ast>) -> Self {
        Value::Array(Arr(value))
    }
}

impl From<Func> for Value {
    fn from(value: Func) -> Self {
        Value::Func(value)
    }
}

impl From<Var> for Value {
    fn from(value: Var) -> Self {
        Value::AttrVar(value)
    }
}

impl From<Arr> for Value {
    fn from(value: Arr) -> Self {
        Value::Array(value)
    }
}

impl From<Map> for Value {
    fn from(value: Map) -> Self {
        Value::Map(value)
    }
}
