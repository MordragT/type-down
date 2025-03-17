use ecow::EcoString;
use tyd_util::TryAsRef;

use crate::{ir, value::Value};

#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub parent: Option<Box<Self>>,
    pub scope: ir::Map,
}

impl Scope {
    pub fn new(from: Self) -> Self {
        Self {
            parent: Some(Box::new(from)),
            scope: ir::Map::new(),
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with<T>(mut self, name: impl Into<EcoString>, value: T) -> Self
    where
        T: Into<Value>,
    {
        self.insert(name.into(), value);
        self
    }

    pub fn clear(&mut self) {
        self.scope.clear();
    }

    pub fn lookup<T>(&self, name: &EcoString) -> Option<T>
    where
        Value: TryAsRef<T>,
        T: Clone,
    {
        self.scope
            .get(name)
            .and_then(|value| value.try_as_ref().cloned())
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.lookup(name)))
    }

    pub fn lookup_str(&self, name: &EcoString) -> Option<EcoString> {
        self.lookup::<EcoString>(name)
    }

    pub fn insert<T>(&mut self, name: EcoString, value: T) -> Option<Value>
    where
        T: Into<Value>,
    {
        self.scope.insert(name, value.into())
    }

    pub fn remove(&mut self, name: &EcoString) -> Option<Value> {
        self.scope.remove(name)
    }

    pub fn extend(&mut self, iter: impl IntoIterator<Item = (EcoString, Value)>) {
        self.scope.extend(iter);
    }

    pub fn enter(&mut self) {
        let parent = std::mem::replace(self, Self::empty());
        self.parent = Some(Box::new(parent));
    }

    pub fn exit(&mut self) -> Self {
        let mut parent = self.parent.take().unwrap();
        std::mem::swap(self, &mut parent);
        *parent
    }
}

// /// A stack of scopes.
// #[derive(Debug, Clone)]
// pub struct Scopes {
//     scopes: Vec<Scope>,
//     base: Arc<Scope>,
// }

// impl Scopes {
//     pub fn new(base: Arc<Scope>) -> Self {
//         Self {
//             base,
//             scopes: vec![Scope::new()],
//         }
//     }

//     pub fn with_scope(base: Arc<Scope>, scope: Scope) -> Self {
//         Self {
//             base,
//             scopes: vec![scope],
//         }
//     }

//     pub fn enter(&mut self) {
//         self.scopes.push(Scope::new())
//     }

//     pub fn exit(&mut self) {
//         self.scopes.pop().expect("no active scope");
//     }

//     pub fn symbol(&self, name: impl AsRef<str>) -> Option<Value> {
//         let name = name.as_ref();

//         self.scopes
//             .iter()
//             .rev()
//             .chain(std::iter::once(self.base.as_ref()))
//             .find_map(|scope| scope.symbol(name))
//     }

//     pub fn func(&self, name: impl AsRef<str>) -> Option<ir::Func> {
//         let name = name.as_ref();

//         self.scopes
//             .iter()
//             .rev()
//             .chain(std::iter::once(self.base.as_ref()))
//             .find_map(|scope| scope.func(name))
//     }

//     pub fn define_symbol<N, V>(&mut self, name: N, value: V) -> Option<Value>
//     where
//         N: Into<EcoString>,
//         V: Into<Value>,
//     {
//         self.scopes
//             .last_mut()
//             .expect("no active scope")
//             .define_symbol(name, value)
//     }

//     pub fn define_func<N, F>(&mut self, name: N, func: F) -> Option<Value>
//     where
//         N: Into<EcoString>,
//         F: Into<ir::Func>,
//     {
//         self.scopes
//             .last_mut()
//             .expect("no active scope")
//             .define_func(name, func)
//     }
// }

// /// A scoped table binding names to values.
// #[derive(Debug, Clone)]
// pub struct Scope {
//     symbols: Map,
// }

// impl Scope {
//     pub fn new() -> Self {
//         Self {
//             symbols: Map::new(),
//         }
//     }

//     pub fn symbol(&self, name: impl AsRef<str>) -> Option<Value> {
//         self.symbols.get(name.as_ref()).cloned()
//     }

//     pub fn func(&self, name: impl AsRef<str>) -> Option<ir::Func> {
//         self.symbols
//             .get(name.as_ref())
//             .cloned()
//             .and_then(|value| value.into_func())
//     }

//     pub fn register_symbol<N, V>(mut self, name: N, value: V) -> Self
//     where
//         N: Into<EcoString>,
//         V: Into<Value>,
//     {
//         self.define_symbol(name, value);
//         self
//     }

//     pub fn register_func<N, F>(mut self, name: N, func: F) -> Self
//     where
//         N: Into<EcoString>,
//         F: Into<ir::Func>,
//     {
//         self.define_func(name, func);
//         self
//     }

//     pub fn define_symbol<N, V>(&mut self, name: N, value: V) -> Option<Value>
//     where
//         N: Into<EcoString>,
//         V: Into<Value>,
//     {
//         self.symbols.insert(name.into(), value.into())
//     }

//     pub fn define_func<N, F>(&mut self, name: N, func: F) -> Option<Value>
//     where
//         N: Into<EcoString>,
//         F: Into<ir::Func>,
//     {
//         self.symbols.insert(name.into(), Value::Func(func.into()))
//     }
// }
