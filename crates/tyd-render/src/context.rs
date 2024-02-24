use crate::{Map, Object};

pub type Args = Map<String, Object>;
pub type Func = Box<dyn Fn(&Args) -> Object>;

pub type SymbolTable = Map<String, Object>;
pub type FunctionTable = Map<String, Func>;

// TODO font-family etc.

pub struct Context {
    function_table: FunctionTable,
    symbol_table: SymbolTable,
}

impl Context {
    pub fn new() -> Self {
        Self {
            function_table: FunctionTable::new(),
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn function(
        mut self,
        key: impl Into<String>,
        f: impl Fn(&Args) -> Object + 'static,
    ) -> Self {
        self.function_table.insert(key.into(), Box::new(f));
        self
    }

    pub fn call(&self, key: impl AsRef<str>) -> Option<&Func> {
        self.function_table.get(key.as_ref())
    }

    pub fn symbol(mut self, key: impl Into<String>, value: impl Into<Object>) -> Self {
        self.symbol_table.insert(key.into(), value.into());
        self
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<&Object> {
        self.symbol_table.get(key.as_ref())
    }
}
