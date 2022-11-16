use crate::ast::Value;
use dashmap::DashMap;

pub struct Context {
    pub values: DashMap<String, Value>,
}

impl Context {
    pub fn insert_value(&self, key: String, value: Value) {
        self.values.insert(key, value);
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            values: DashMap::new(),
        }
    }
}
