use crate::{ast::LangIndexVar, Ast, Context, RuntimeError, Value, ValueResult};

impl LangIndexVar for Vec<Ast> {
    fn get_index(&self, ctx: &Context, index: &Ast) -> ValueResult {
        match index.execute(ctx) {
            Ok(value) => match value {
                Value::Int(index) => match self.get(index as usize) {
                    Some(item) => item.execute(ctx),
                    None => Err(RuntimeError::IndexOutOfBounds(index)),
                },
                _ => Err(RuntimeError::NotIndexable(value)),
            },
            Err(err) => Err(err),
        }
    }
}
