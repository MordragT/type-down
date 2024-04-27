use std::sync::Arc;

use crate::world::World;

use super::{Engine, Machine, Scope};

pub struct Context<E: Engine> {
    pub world: World<E>,
    pub scope: Scope<E>,
}

impl<E: Engine> Context<E> {
    pub fn new(world: World<E>) -> Self {
        Self {
            world,
            scope: Scope::new(),
        }
    }

    pub fn with_scope(world: World<E>, scope: Scope<E>) -> Self {
        Self { world, scope }
    }

    pub fn forge<'a>(&'a mut self, engine: &'a mut E) -> Machine<E> {
        Machine {
            engine,
            scope: &mut self.scope,
            world: self.world.clone(),
        }
    }
}
