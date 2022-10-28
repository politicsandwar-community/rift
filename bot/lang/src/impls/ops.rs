use std::ops::{Add, Div, Mul, Neg, Not, Rem, Sub};

use bigdecimal::BigDecimal;

use crate::{
    ast::{Op, ToBool},
    RuntimeError, Value, ValueResult,
};

impl Value {
    pub fn and(&self, other: &Value) -> bool {
        if self.to_bool() {
            other.to_bool()
        } else {
            false
        }
    }

    pub fn or(&self, other: &Value) -> bool {
        if self.to_bool() {
            true
        } else {
            other.to_bool()
        }
    }

    pub fn floor(self, rhs: Value) -> ValueResult {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Ok((a / b).into()),
            (Value::Int(a), Value::Float(b)) => Ok((((a as f64) / b) as i64).into()),
            // (Value::Int(a), Value::Decimal(b)) => Ok(((a / b) as i64).into()),
            (Value::Float(a), Value::Float(b)) => Ok(((a / b) as i64).into()),
            (Value::Float(a), Value::Int(b)) => Ok(((a / (b as f64)) as i64).into()),

            // (Value::Decimal(a), Value::Decimal(b)) => Ok(((a / b).to_i64()).into()),
            // (Value::Decimal(a), Value::Int(b)) => Ok(((a / (b as BigDecimal)).to_i64()).into()),
            (a, b) => Err(RuntimeError::InvalidOp(Op::Floor, Some(a), b)),
        }
    }

    pub fn pow(self, rhs: Value) -> ValueResult {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Ok((a.pow(b as u32)).into()),
            (Value::Int(a), Value::Float(b)) => Ok(((a as f64).powf(b)).into()),

            (Value::Float(a), Value::Float(b)) => Ok((a.powf(b)).into()),
            (Value::Float(a), Value::Int(b)) => Ok(a.powf(b as f64).into()),

            (a, b) => Err(RuntimeError::InvalidOp(Op::Pow, Some(a), b)),
        }
    }
}

impl Add for Value {
    type Output = ValueResult;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Ok((a + b).into()),
            (Value::Int(a), Value::Float(b)) => Ok((a as f64 + b).into()),
            (Value::Int(a), Value::Decimal(b)) => Ok((b.add(BigDecimal::from(a))).into()),

            (Value::Float(a), Value::Float(b)) => Ok((a + b).into()),
            (Value::Float(a), Value::Int(b)) => Ok((a + b as f64).into()),

            (Value::Decimal(a), Value::Decimal(b)) => Ok((a + b).into()),
            (Value::Decimal(a), Value::Int(b)) => Ok((a + BigDecimal::from(b)).into()),

            (Value::String(a), Value::String(b)) => Ok((a + &b).into()),
            (Value::Array(mut a), Value::Array(b)) => {
                a.0.extend(b.0);
                Ok(a.into())
            },
            (Value::Map(mut a), Value::Map(b)) => {
                a.0.extend(b.0);
                Ok(a.into())
            },
            (a, b) => Err(RuntimeError::InvalidOp(Op::Add, Some(a), b)),
        }
    }
}

impl Sub for Value {
    type Output = ValueResult;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Ok((a - b).into()),
            (Value::Int(a), Value::Float(b)) => Ok((a as f64 - b).into()),
            (Value::Int(a), Value::Decimal(b)) => Ok((BigDecimal::from(a) - b).into()),

            (Value::Float(a), Value::Float(b)) => Ok((a - b).into()),
            (Value::Float(a), Value::Int(b)) => Ok((a - b as f64).into()),

            (Value::Decimal(a), Value::Decimal(b)) => Ok((a - b).into()),
            (Value::Decimal(a), Value::Int(b)) => Ok((a - BigDecimal::from(b)).into()),

            (a, b) => Err(RuntimeError::InvalidOp(Op::Sub, Some(a), b)),
        }
    }
}

impl Mul for Value {
    type Output = ValueResult;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Ok((a * b).into()),
            (Value::Int(a), Value::Float(b)) => Ok((a as f64 * b).into()),
            (Value::Int(a), Value::Decimal(b)) => Ok((b * BigDecimal::from(a)).into()),

            (Value::Float(a), Value::Float(b)) => Ok((a * b).into()),
            (Value::Float(a), Value::Int(b)) => Ok((a * b as f64).into()),

            (Value::Decimal(a), Value::Decimal(b)) => Ok((a * b).into()),
            (Value::Decimal(a), Value::Int(b)) => Ok((a * BigDecimal::from(b)).into()),

            (Value::String(a), Value::Int(b)) => Ok((a.repeat(b as usize)).into()),

            (Value::Array(a), Value::Int(b)) => {
                let mut result = Vec::with_capacity(a.0.len() * b as usize);
                for _ in 0..b {
                    result.extend(a.0.clone());
                }
                Ok(result.into())
            },

            (a, b) => Err(RuntimeError::InvalidOp(Op::Mul, Some(a), b)),
        }
    }
}

impl Div for Value {
    type Output = ValueResult;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs == Value::Int(0)
            || rhs == Value::Float(0.0)
            || rhs == Value::Decimal(BigDecimal::from(0))
        {
            return Err(RuntimeError::DivisionByZero);
        }
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Ok((a / b).into()),
            (Value::Int(a), Value::Float(b)) => Ok((a as f64 / b).into()),
            (Value::Int(a), Value::Decimal(b)) => Ok((BigDecimal::from(a) / b).into()),

            (Value::Float(a), Value::Float(b)) => Ok((a / b).into()),
            (Value::Float(a), Value::Int(b)) => Ok((a / b as f64).into()),

            (Value::Decimal(a), Value::Decimal(b)) => Ok((a / b).into()),
            (Value::Decimal(a), Value::Int(b)) => Ok((a / BigDecimal::from(b)).into()),

            (a, b) => Err(RuntimeError::InvalidOp(Op::Div, Some(a), b)),
        }
    }
}

impl Rem for Value {
    type Output = ValueResult;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Ok((a % b).into()),
            (Value::Int(a), Value::Float(b)) => Ok((a as f64 % b).into()),
            (Value::Int(a), Value::Decimal(b)) => Ok((BigDecimal::from(a) % b).into()),

            (Value::Float(a), Value::Float(b)) => Ok((a % b).into()),
            (Value::Float(a), Value::Int(b)) => Ok((a % b as f64).into()),

            (Value::Decimal(a), Value::Decimal(b)) => Ok((a % b).into()),
            (Value::Decimal(a), Value::Int(b)) => Ok((a % BigDecimal::from(b)).into()),

            (a, b) => Err(RuntimeError::InvalidOp(Op::Mod, Some(a), b)),
        }
    }
}

impl Not for Value {
    type Output = ValueResult;

    fn not(self) -> Self::Output {
        Ok((!self.to_bool()).into())
    }
}

impl Neg for Value {
    type Output = ValueResult;

    fn neg(self) -> Self::Output {
        match self {
            Value::Int(a) => Ok((-a).into()),
            Value::Float(a) => Ok((-a).into()),
            Value::Decimal(a) => Ok((-a).into()),
            a => Err(RuntimeError::InvalidOp(Op::Neg, None, a)),
        }
    }
}
