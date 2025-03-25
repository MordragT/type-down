use chumsky::prelude::*;
use tyd_core::prelude::*;

use crate::{LocationPhase, Span};

/// Represents the parser state using a simple state wrapper around `StateRepr`.
pub type State = extra::SimpleState<StateRepr>;

/// Represents extra data passed to the parser, combining rich error reporting
/// with state and context information.
///
/// The lifetime parameter `'src` corresponds to the source text being parsed.
pub type Extra<'src> = extra::Full<Rich<'src, char, Span>, State, Context>;

/// Provides contextual information for the parser.
///
/// This structure maintains state information needed during parsing,
/// such as the current indentation level.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Context {
    /// The current indentation level in spaces.
    pub indent: usize,
}

/// Represents the internal state for document building during parsing.
///
/// This structure maintains both the document being constructed and
/// associated metadata for syntax elements.
#[derive(Debug, Clone, Default)]
pub struct StateRepr {
    /// The document builder used to construct the AST.
    pub builder: DocBuilder,

    /// Collection of metadata associated with syntax elements.
    pub meta: MetaVec<LocationPhase>,
}

impl StateRepr {
    /// Creates a new empty `StateRepr` with default values.
    ///
    /// # Returns
    /// A fresh `StateRepr` instance with empty builder and metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a node into the document and records its metadata.
    ///
    /// # Parameters
    /// * `node` - The node to insert into the document
    /// * `meta` - Span information for the node
    ///
    /// # Returns
    /// A unique identifier for the inserted node
    ///
    /// # Type Parameters
    /// * `T` - Node type that can be cast to metadata and converted to a `Node`
    pub fn insert<T>(&mut self, node: T, meta: Span) -> NodeId<T>
    where
        T: MetaCast<LocationPhase, Meta = Span>,
        Node: From<T>,
    {
        let id = self.builder.insert(node);
        self.meta.push(T::upcast(meta));

        id
    }

    /// Retrieves a reference to a node by its ID.
    ///
    /// # Parameters
    /// * `id` - The identifier of the node to retrieve
    ///
    /// # Returns
    /// A reference to the node
    ///
    /// # Type Parameters
    /// * `T` - The expected type of the node
    pub fn node<T>(&self, id: NodeId<T>) -> &T
    where
        Node: TryAsRef<T>,
    {
        self.builder.node(id)
    }
}
