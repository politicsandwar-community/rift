use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Not, Rem, Sub},
    sync::Arc,
};

use bigdecimal::BigDecimal;
use dashmap::DashMap;

use crate::{error::RuntimeError, Context, Expose};

type BoxedAst = Box<Ast>;

#[derive(Debug, Clone)]
pub enum Op {
    And,
    Or,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    Floor,
    Mod,
    Pow,
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum Ast {
    Ident(Option<BoxedAst>, String),
    Static(Option<BoxedAst>, String),
    Index(BoxedAst, BoxedAst),
    Call(BoxedAst, Vec<Ast>),
    Literal(Value),
    Expr(Option<BoxedAst>, Op, BoxedAst),
}

impl Ast {
    pub fn execute(&self, ctx: &Context) -> ValueResult {
        match self {
            Ast::Ident(ast, ident) => match ast.as_ref() {
                Some(ast) => match ast.as_ref() {
                    Ast::Ident(_, ident_ident) => match ast.execute(ctx) {
                        Ok(value) => match value {
                            Value::Int(val) => val.get_attr(ctx, ident_ident),
                            Value::Float(val) => val.get_attr(ctx, ident_ident),
                            Value::Decimal(val) => val.get_attr(ctx, ident_ident),
                            Value::Bool(val) => val.get_attr(ctx, ident_ident),
                            Value::String(val) => val.get_attr(ctx, ident_ident),
                            Value::AttrVar(val) => val.0.get_attr(ctx, ident_ident),
                            _ => Err(RuntimeError::AttributeNotFound(ident_ident.clone())),
                        },
                        Err(err) => Err(err),
                    },
                    _ => Err(RuntimeError::AttributeNotFound(ident.clone())),
                },
                None => match ctx.values.get(ident) {
                    Some(var) => Ok(var.value().clone()),
                    None => Err(RuntimeError::VariableNotFound(ident.clone())),
                },
            },
            Ast::Static(ast, ident) => match ast.as_ref() {
                Some(ast) => match ast.as_ref() {
                    Ast::Ident(_, ident_ident) => match ast.execute(ctx) {
                        Ok(value) => match value {
                            Value::Int(val) => val.get_static_attr(ctx, ident_ident),
                            Value::Float(val) => val.get_static_attr(ctx, ident_ident),
                            Value::Decimal(val) => val.get_static_attr(ctx, ident_ident),
                            Value::Bool(val) => val.get_static_attr(ctx, ident_ident),
                            Value::String(val) => val.get_static_attr(ctx, ident_ident),
                            Value::AttrVar(val) => val.0.get_static_attr(ctx, ident_ident),
                            _ => Err(RuntimeError::AttributeNotFound(ident_ident.clone())),
                        },
                        Err(err) => Err(err),
                    },
                    _ => Err(RuntimeError::AttributeNotFound(ident.clone())),
                },
                None => match ctx.values.get(ident) {
                    Some(var) => Ok(var.value().clone()),
                    None => Err(RuntimeError::VariableNotFound(ident.clone())),
                },
            },
            Ast::Index(ident, _) => match ident.execute(ctx) {
                Ok(value) => Err(RuntimeError::NotIndexable(value)),
                Err(err) => Err(err),
            },
            Ast::Call(ast, args) => match ast.execute(ctx) {
                Ok(value) => match value {
                    Value::Func(func) => {
                        let args = args
                            .iter()
                            .map(|arg| arg.execute(ctx))
                            .collect::<Vec<ValueResult>>();
                        if let Some(err) = args.iter().find(|arg| arg.is_err()) {
                            let e = err.as_ref().unwrap_err();
                            Err(e.to_owned())
                        } else {
                            func.call(
                                ctx,
                                args.iter()
                                    .map(|arg| arg.as_ref().unwrap().to_owned())
                                    .collect::<Vec<Value>>(),
                            )
                        }
                    },
                    _ => Err(RuntimeError::NotCallable(value)),
                },
                Err(err) => Err(err),
            },
            Ast::Literal(value) => Ok(value.clone()),
            Ast::Expr(lhs_ast, op, rhs_ast) => {
                let rhs = rhs_ast.execute(ctx)?;

                let lhs = match lhs_ast {
                    Some(ast) => match ast.execute(ctx) {
                        Ok(value) => Some(value),
                        Err(err) => return Err(err),
                    },
                    None => None,
                };

                match op {
                    Op::And => Ok(lhs.unwrap().and(&rhs).into()),
                    Op::Or => Ok(lhs.unwrap().or(&rhs).into()),
                    Op::Eq => Ok(lhs.unwrap().eq(&rhs).into()),
                    Op::Ne => Ok(lhs.unwrap().ne(&rhs).into()),
                    Op::Lt => Ok(lhs.unwrap().lt(&rhs).into()),
                    Op::Le => Ok(lhs.unwrap().le(&rhs).into()),
                    Op::Gt => Ok(lhs.unwrap().gt(&rhs).into()),
                    Op::Ge => Ok(lhs.unwrap().ge(&rhs).into()),
                    Op::Add => lhs.unwrap().add(rhs),
                    Op::Sub => lhs.unwrap().sub(rhs),
                    Op::Mul => lhs.unwrap().mul(rhs),
                    Op::Div => lhs.unwrap().div(rhs),
                    Op::Floor => lhs.unwrap().floor(rhs),
                    Op::Mod => lhs.unwrap().rem(rhs),
                    Op::Pow => lhs.unwrap().pow(rhs),
                    Op::Not => rhs.not(),
                    Op::Neg => rhs.neg(),
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Int(i64),
    Float(f64),
    Decimal(BigDecimal),
    Bool(bool),
    String(String),
    Func(Func),
    Map(Map),
    Array(Arr),
    AttrVar(Var),
    None,
}

type FuncFn = dyn Fn(&Context, Vec<Value>) -> ValueResult;

pub struct Func(pub Arc<FuncFn>);

impl Func {
    pub fn new(func: impl Fn(&Context, Vec<Value>) -> ValueResult + 'static) -> Self {
        Self(Arc::new(func))
    }

    pub fn call(&self, ctx: &Context, args: Vec<Value>) -> ValueResult {
        (self.0.as_ref())(ctx, args)
    }
}

impl Clone for Func {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Func")
    }
}

impl PartialEq for Func {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd for Func {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

#[derive(Debug)]
pub struct Var(pub Arc<dyn Expose>);

impl Var {
    pub fn new(var: impl Expose + 'static) -> Self {
        Self(Arc::new(var))
    }
}

impl Clone for Var {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for Var {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd for Var {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

pub trait LangIndexVar {
    fn get_index(&self, ctx: &Context, index: &Ast) -> ValueResult;
}

pub type ValueResult = Result<Value, RuntimeError>;

pub trait ToBool {
    fn to_bool(&self) -> bool;
}

#[derive(Debug)]
pub struct Arr(pub Vec<Ast>);

impl PartialEq for Arr {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd for Arr {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

impl Clone for Arr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[derive(Debug)]
pub struct Map(pub DashMap<String, Ast>);

impl PartialEq for Map {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd for Map {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}

impl Clone for Map {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
