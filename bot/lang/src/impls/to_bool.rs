use crate::{ast::ToBool, Value};

impl ToBool for Value {
    fn to_bool(&self) -> bool {
        match self {
            Value::Int(value) => *value != 0,
            Value::Float(value) => *value != 0.0,
            Value::Decimal(value) => value != &0.into(),
            Value::Bool(value) => *value,
            Value::String(value) => !value.is_empty(),
            Value::Func(_) => true,
            Value::Map(value) => !value.0.is_empty(),
            Value::Array(value) => !value.0.is_empty(),
            Value::AttrVar(_) => true,
            Value::None => false,
        }
    }
}
