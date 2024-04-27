use std::sync::Arc;

use super::{Engine, Scope};
use crate::{foundations::Func, value::Value, world::World};

/// Holds the state needed for evaluation.
pub struct Machine<'a, E: Engine> {
    pub engine: &'a mut E,
    pub world: World<E>,
    pub scope: &'a mut Scope<E>,
}

impl<'a, E: Engine> Machine<'a, E> {
    pub fn symbol(&self, name: impl AsRef<str>) -> Option<Value<E>> {
        let name = name.as_ref();

        self.scope
            .symbol(name)
            .or(self.world.global_scope().symbol(name))
    }

    pub fn func(&self, name: impl AsRef<str>) -> Option<Arc<dyn Func<E>>> {
        let name = name.as_ref();

        self.scope
            .func(name)
            .or(self.world.global_scope().func(name))
    }
}
