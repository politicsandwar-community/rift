use bigdecimal::BigDecimal;

use crate::{
    ast::{ExposeVar, Func, ValueResult},
    error::RuntimeError,
    Context, Value,
};

impl ExposeVar for i64 {
    fn get_attr<'a>(&'a self, _ctx: &Context, ident: &str) -> ValueResult {
        let cloned_self = *self;
        match ident {
            "to_string" => {
                Ok(Func::new(move |_ctx, _args| Ok(cloned_self.to_string().into())).into())
            },
            _ => Err(RuntimeError::AttributeNotFound(ident.to_string())),
        }
    }
}

impl ExposeVar for f64 {
    fn get_attr(&self, _ctx: &Context, ident: &str) -> ValueResult {
        let cloned_self = *self;
        match ident {
            "to_string" => {
                Ok(Func::new(move |_ctx, _args| Ok(Value::String(cloned_self.to_string()))).into())
            },
            _ => Err(RuntimeError::AttributeNotFound(ident.to_string())),
        }
    }
}

impl ExposeVar for BigDecimal {
    fn get_attr(&self, _ctx: &Context, ident: &str) -> ValueResult {
        let cloned_self = self.clone();
        match ident {
            "to_string" => {
                Ok(Func::new(move |_ctx, _args| Ok(Value::String(cloned_self.to_string()))).into())
            },
            _ => Err(RuntimeError::AttributeNotFound(ident.to_string())),
        }
    }
}

impl ExposeVar for bool {
    fn get_attr(&self, _ctx: &Context, ident: &str) -> ValueResult {
        let cloned_self = *self;
        match ident {
            "to_string" => {
                Ok(Func::new(move |_ctx, _args| Ok(Value::String(cloned_self.to_string()))).into())
            },
            _ => Err(RuntimeError::AttributeNotFound(ident.to_string())),
        }
    }
}

impl ExposeVar for String {
    fn get_attr(&self, _ctx: &Context, ident: &str) -> ValueResult {
        let cloned_self = self.clone();
        match ident {
            "to_string" => {
                Ok(Func::new(move |_ctx, _args| Ok(Value::String(cloned_self.to_string()))).into())
            },
            _ => Err(RuntimeError::AttributeNotFound(ident.to_string())),
        }
    }
}
