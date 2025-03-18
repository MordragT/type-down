use chumsky::prelude::*;
use tyd_core::prelude::*;

use crate::{Span, SyntaxPhase};

pub type State = extra::SimpleState<StateRepr>;
pub type Extra<'src> = extra::Full<Rich<'src, char, Span>, State, Context>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Context {
    pub indent: usize,
}

#[derive(Debug, Clone, Default)]
pub struct StateRepr {
    pub builder: DocBuilder,
    pub meta: MetaVec<SyntaxPhase>,
}

impl StateRepr {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T>(&mut self, node: T, meta: Span) -> NodeId<T>
    where
        T: MetaCast<SyntaxPhase, Meta = Span>,
        Node: From<T>,
    {
        let id = self.builder.insert(node);
        self.meta.push(T::upcast(meta));

        id
    }

    pub fn node<T>(&self, id: NodeId<T>) -> &T
    where
        Node: TryAsRef<T>,
    {
        self.builder.node(id)
    }
}
