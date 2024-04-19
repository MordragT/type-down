use std::collections::BTreeMap;

use tyd_syntax::ast::{Arg, Call, Expr, Inline};

use crate::{builtin, error::ContextError, Value};

// TODO create Function trait

type Map<K, V> = BTreeMap<K, V>;
pub type Args<C> = Map<String, Value<C>>;
pub type Func<C> = Box<dyn Fn(Args<C>) -> Result<Value<C>, ContextError>>;

pub type SymbolTable<C> = Map<String, Value<C>>;
pub type FunctionTable<C> = Map<String, Func<C>>;

// TODO font-family etc.

pub struct Context<C> {
    function_table: FunctionTable<C>,
    symbol_table: SymbolTable<C>,
}

impl<C: Clone + 'static> Context<C> {
    pub fn new() -> Self {
        let mut function_table = FunctionTable::new();

        function_table.insert("dict".into(), Box::new(builtin::dict));
        function_table.insert("list".into(), Box::new(builtin::list));

        Self {
            function_table,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn function(
        mut self,
        key: impl Into<String>,
        f: impl Fn(Args<C>) -> Result<Value<C>, ContextError> + 'static,
    ) -> Self {
        self.function_table.insert(key.into(), Box::new(f));
        self
    }

    pub fn symbol(mut self, key: impl Into<String>, value: impl Into<Value<C>>) -> Self {
        self.symbol_table.insert(key.into(), value.into());
        self
    }

    pub fn eval(&self, expr: &Expr) -> Result<Value<C>, ContextError> {
        match expr {
            Expr::Block(block) => todo!(),
            Expr::Call(call) => self.eval_call(call),
            Expr::Ident(ident) => self.eval_symbol(ident),
            Expr::Literal(literal) => Ok(Value::from(literal.to_owned())),
        }
    }

    pub fn eval_symbol(&self, key: impl AsRef<str>) -> Result<Value<C>, ContextError> {
        self.symbol_table
            .get(key.as_ref())
            .cloned()
            .ok_or(ContextError::SymbolNotFound(key.as_ref().to_owned()))
    }

    pub fn eval_call(&self, call: &Call) -> Result<Value<C>, ContextError> {
        let Call {
            ident,
            args,
            content,
        } = call;

        let f = self
            .function_table
            .get(ident.as_str())
            .ok_or(ContextError::FunctionNotFound(ident.to_string()))?;

        let args = self.eval_args(args, content)?;

        f(args)
    }

    pub fn eval_args(
        &self,
        args: &Vec<Arg>,
        content: &Option<Vec<Inline>>,
    ) -> Result<Args<C>, ContextError> {
        let mut evaluated = Args::new();

        for arg in args {
            // TODO use the position of the arg and a Function trait wich has a method args -> &[&str] where
            // the args are shown in correct order to get the name if not specified.
            let name = arg.name.as_ref().unwrap().to_string();
            let value = self.eval(&arg.value)?;

            evaluated.insert(name, value);
        }

        if let Some(content) = content {
            // evaluated.insert("content".to_owned(), Value::Content(content));

            // TODO need some kind of callback or virtual machine to evaluated the inline content
            todo!()
        }

        Ok(evaluated)
    }
}
