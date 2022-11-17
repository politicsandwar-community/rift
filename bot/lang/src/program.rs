use std::sync::Arc;

use crate::Context;
use crate::{ast::Ast, lang::ExpressionParser};
use crate::{ast::ValueResult, CompileError};
use lalrpop_util::{lexer::Token, ParseError};

#[derive(Debug, Clone)]
pub struct Program {
    ast: Arc<Ast>,
}

impl Program {
    pub fn compile(source: &'_ str) -> Result<Program, ParseError<usize, Token<'_>, CompileError>> {
        match ExpressionParser::new().parse(source) {
            Ok(ast) => Ok(Program { ast: Arc::new(ast) }),
            Err(err) => Err(err),
        }
    }

    pub fn execute(&self, ctx: &Context) -> ValueResult {
        self.ast.execute(ctx)
    }
}
