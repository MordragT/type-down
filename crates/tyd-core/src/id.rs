use std::{hash::Hash, marker::PhantomData};

use crate::meta::{MetaCast, MetaContainer, Phase};

/// A strongly-typed identifier for nodes.
///
/// The `NodeId<T>` type provides a type-safe way to reference nodes of type `T`
/// while maintaining compile-time type checking. Internally, it's represented as
/// a simple 32-bit unsigned integer.
#[derive(Debug)]
pub struct NodeId<T> {
    /// The underlying numeric identifier
    id: u32,
    /// Phantom type parameter for type safety without runtime overhead
    t: PhantomData<T>,
}

impl<T> Clone for NodeId<T> {
    /// Creates a copy of this `NodeId`.
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            t: PhantomData,
        }
    }
}

/// Enables `NodeId<T>` to be copied rather than moved.
impl<T> Copy for NodeId<T> {}

impl<T> PartialEq for NodeId<T> {
    /// Compares two `NodeId`s for equality.
    ///
    /// Two `NodeId`s are equal if they have the same internal ID value.
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

/// Implements total equality for `NodeId<T>`.
impl<T> Eq for NodeId<T> {}

impl<T> PartialOrd for NodeId<T> {
    /// Provides partial ordering for `NodeId` instances based on their internal ID values.
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T> Ord for NodeId<T> {
    /// Provides total ordering for `NodeId` instances based on their internal ID values.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T> Hash for NodeId<T> {
    /// Implements hashing for `NodeId<T>` by hashing the internal ID value.
    ///
    /// This allows `NodeId`s to be used as keys in hash maps and sets.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> NodeId<T> {
    /// Creates a new `NodeId` with the specified numeric identifier.
    ///
    /// This constructor is crate-private to prevent external code from creating
    /// arbitrary node IDs.
    pub(crate) fn new(id: u32) -> Self {
        Self { id, t: PhantomData }
    }

    /// Converts the node ID to a `usize` value.
    ///
    /// This is useful when using the ID as an index into a vector or similar collection.
    pub fn as_usize(&self) -> usize {
        self.id as usize
    }

    /// Retrieves metadata associated with this node from a metadata container.
    ///
    /// # Type Parameters
    /// * `P` - The phase of processing that the metadata belongs to
    ///
    /// # Parameters
    /// * `metadata` - A container implementing `MetaContainer<P>` that holds node metadata
    ///
    /// # Returns
    /// A reference to the metadata associated with this node
    pub fn meta<P>(self, metadata: &impl MetaContainer<P>) -> &T::Meta
    where
        P: Phase,
        T: MetaCast<P>,
    {
        metadata.meta(self)
    }
}
