use crate::ast::ValueResult;
use crate::Context;
use crate::{ast::Ast, lang::ExpressionParser};
use lalrpop_util::{lexer::Token, ParseError};

#[derive(Debug, Clone)]
pub struct Program {
    ast: Ast,
}

impl Program {
    pub fn compile(source: &'_ str) -> Result<Program, ParseError<usize, Token<'_>, &'_ str>> {
        match ExpressionParser::new().parse(source) {
            Ok(ast) => Ok(Program { ast }),
            Err(err) => Err(err),
        }
    }

    pub fn execute(&self, ctx: &Context) -> ValueResult {
        self.ast.execute(ctx)
    }
}
