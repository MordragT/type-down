use std::sync::Arc;

use crate::{Command, Shape, Value};

pub trait SymbolTable<S: Shape> {
    fn symbol(&self, key: impl AsRef<str>) -> Option<Value<S>>;
    fn command(&self, key: impl AsRef<str>) -> Option<Arc<dyn Command<S>>>;
}
