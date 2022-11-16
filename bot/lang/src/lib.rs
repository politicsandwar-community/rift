#[macro_use]
extern crate lalrpop_util;

mod ast;
mod context;
mod error;
mod impls;
mod program;

use std::fmt::Debug;

pub use crate::ast::{Ast, Func, LangIndexVar, Value, ValueResult, Var};
pub use crate::context::Context;
pub use crate::error::{CompileError, RuntimeError, TypeError};
pub use crate::program::Program;

lalrpop_mod!(lang);

pub trait Expose: Debug {
    fn get_attr(&self, _ctx: &Context, ident: &str) -> ValueResult {
        Err(RuntimeError::AttributeNotFound(ident.to_string()))
    }

    fn get_static_attr(&self, _ctx: &Context, ident: &str) -> ValueResult {
        Err(RuntimeError::AttributeNotFound(ident.to_string()))
    }
}

pub trait Enum {
    fn from_i16(value: i16) -> Option<Box<Self>>;

    fn to_i16(&self) -> i16;
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    use super::*;

    #[test]
    fn it_works() {
        let ctx = Context::default();
        ctx.insert_value("test".into(), 1.into());
        assert_eq!(
            Program::compile("1 + 2").unwrap().execute(&ctx).unwrap(),
            Value::Int(3)
        );
        assert_eq!(
            Program::compile("1 + 2 * 3")
                .unwrap()
                .execute(&ctx)
                .unwrap(),
            Value::Int(7)
        );
        assert_eq!(
            Program::compile("1.0 + 2 * 3 + 4")
                .unwrap()
                .execute(&ctx)
                .unwrap(),
            Value::Float(11.0)
        );
        assert_eq!(
            Program::compile("true && false")
                .unwrap()
                .execute(&ctx)
                .unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            Program::compile("true || false")
                .unwrap()
                .execute(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            Program::compile("true || false && true")
                .unwrap()
                .execute(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            Program::compile("[1, 2, 3][1]")
                .unwrap()
                .execute(&ctx)
                .unwrap(),
            Value::Int(2)
        );
        assert_eq!(
            Program::compile("[1, 2] && true")
                .unwrap()
                .execute(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            Program::compile("test + 1 == 2")
                .unwrap()
                .execute(&ctx)
                .unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            Program::compile("d1.5 + 1").unwrap().execute(&ctx).unwrap(),
            Value::Decimal(BigDecimal::from_str("2.5").unwrap())
        );
    }
}
