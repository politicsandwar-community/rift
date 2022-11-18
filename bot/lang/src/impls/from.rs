use bigdecimal::BigDecimal;
use dashmap::DashMap;

use crate::{
    ast::{Arr, Func, Map},
    Ast, Context, Duration, Value, ValueResult, Var,
};

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<&i64> for Value {
    fn from(value: &i64) -> Self {
        Value::Int(*value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value as i64)
    }
}

impl From<&i32> for Value {
    fn from(value: &i32) -> Self {
        Value::Int(*value as i64)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<&f64> for Value {
    fn from(value: &f64) -> Self {
        Value::Float(*value)
    }
}

impl From<BigDecimal> for Value {
    fn from(value: BigDecimal) -> Self {
        Value::Decimal(value)
    }
}

impl From<&BigDecimal> for Value {
    fn from(value: &BigDecimal) -> Self {
        Value::Decimal(value.clone())
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<&bool> for Value {
    fn from(value: &bool) -> Self {
        Value::Bool(*value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        Value::String(value.clone())
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
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

impl<T> From<Option<T>> for Value
where
    Value: From<T>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Value::None,
        }
    }
}

impl<'a, T> From<&'a Option<T>> for Value
where
    Value: std::convert::From<&'a T>,
{
    fn from(value: &'a Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Value::None,
        }
    }
}

impl From<time::OffsetDateTime> for Value {
    fn from(value: time::OffsetDateTime) -> Self {
        Value::Time(value)
    }
}

impl From<&time::OffsetDateTime> for Value {
    fn from(value: &time::OffsetDateTime) -> Self {
        Value::Time(*value)
    }
}

impl From<time::Duration> for Value {
    fn from(value: time::Duration) -> Self {
        Value::Duration(value.into())
    }
}

impl From<&time::Duration> for Value {
    fn from(value: &time::Duration) -> Self {
        Value::Duration(value.into())
    }
}

impl From<Duration> for Value {
    fn from(value: Duration) -> Self {
        Value::Duration(value)
    }
}

impl From<&Duration> for Value {
    fn from(value: &Duration) -> Self {
        Value::Duration(value.clone())
    }
}

impl From<time::Duration> for Duration {
    fn from(value: time::Duration) -> Self {
        Duration {
            seconds: value.whole_seconds(),
        }
    }
}

impl From<&time::Duration> for Duration {
    fn from(value: &time::Duration) -> Self {
        Duration {
            seconds: value.whole_seconds(),
        }
    }
}

impl<T> From<T> for Value
where
    T: Fn(&Context, Vec<Value>) -> ValueResult + Send + Sync + 'static,
{
    fn from(v: T) -> Self {
        Value::Func(Func::new(v))
    }
}
