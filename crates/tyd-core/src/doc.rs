use std::sync::Arc;

use crate::{Full, TryAsMut, TryAsRef, id::NodeId, node::Node, tree::Block, visit::Visitor};

/// A builder for creating a `Doc` instance.
///
/// This struct allows incremental construction of a document by adding nodes
/// and manipulating them before finalizing the document structure.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DocBuilder {
    /// The nodes in the document.
    nodes: Vec<Node>,
}

impl DocBuilder {
    /// Get an immutable reference to a node.
    ///
    /// # Parameters
    /// - `id`: The NodeId of the node to retrieve
    ///
    /// # Returns
    /// A reference to the node of type T
    ///
    /// # Panics
    /// Panics if the node doesn't exist or cannot be converted to type T
    pub fn node<T>(&self, id: NodeId<T>) -> &T
    where
        Node: TryAsRef<T>,
    {
        let node = &self.nodes[id.as_usize()];
        node.try_as_ref().unwrap()
    }

    /// Get a mutable reference to a node.
    ///
    /// # Parameters
    /// - `id`: The NodeId of the node to retrieve
    ///
    /// # Returns
    /// A mutable reference to the node of type T
    ///
    /// # Panics
    /// Panics if the node doesn't exist or cannot be converted to type T
    pub fn node_mut<T>(&mut self, id: NodeId<T>) -> &mut T
    where
        Node: TryAsMut<T>,
    {
        let node = &mut self.nodes[id.as_usize()];
        node.try_as_mut().unwrap()
    }

    /// Update a node, returning the old node.
    ///
    /// # Parameters
    /// - `id`: The NodeId of the node to update
    /// - `node`: The new node value to replace the existing one
    ///
    /// # Returns
    /// The previous node value
    ///
    /// # Panics
    /// Panics if the node doesn't exist or cannot be converted to type T
    pub fn update_node<T>(&mut self, id: NodeId<T>, node: T) -> T
    where
        Node: TryAsMut<T>,
    {
        std::mem::replace(self.node_mut(id), node)
    }

    /// Insert a new node into the builder.
    ///
    /// # Parameters
    /// - `node`: The node to insert into the document
    ///
    /// # Returns
    /// A NodeId that can be used to reference the inserted node
    pub fn insert<T>(&mut self, node: T) -> NodeId<T>
    where
        Node: From<T>,
    {
        let id = self.nodes.len() as u32;

        self.nodes.push(Node::from(node));

        NodeId::new(id)
    }

    /// Finish building the document.
    ///
    /// # Parameters
    /// - `blocks`: A vector of NodeIds representing the top-level blocks in the document
    ///
    /// # Returns
    /// A completed `Doc` instance
    pub fn finish(self, blocks: Vec<NodeId<Block>>) -> Doc {
        let Self { nodes } = self;

        let nodes = Arc::new(nodes);

        Doc { blocks, nodes }
    }
}

/// A document representing a structured tree of nodes.
///
/// The `Doc` struct provides an immutable view of a document with methods to
/// traverse and inspect its content. It maintains a collection of blocks (top-level
/// elements) and nodes (all elements in the document).
#[derive(Debug, Clone, PartialEq)]
pub struct Doc {
    /// The blocks in the document (top-level elements).
    blocks: Vec<NodeId<Block>>,
    /// The nodes in the document (all elements).
    nodes: Arc<Vec<Node>>,
}

impl Doc {
    /// Get an immutable reference to the blocks.
    ///
    /// # Returns
    /// A reference to the vector of top-level block NodeIds
    #[inline]
    pub fn blocks(&self) -> &Vec<NodeId<Block>> {
        &self.blocks
    }

    /// Get an immutable reference to a node by its index.
    ///
    /// # Parameters
    /// - `id`: The raw index of the node
    ///
    /// # Returns
    /// A reference to the node at the specified index
    ///
    /// # Panics
    /// Panics if the index is out of bounds
    #[inline]
    pub fn get(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    /// Get an immutable reference to a node.
    ///
    /// # Parameters
    /// - `id`: The NodeId of the node to retrieve
    ///
    /// # Returns
    /// A reference to the node of type T
    ///
    /// # Panics
    /// Panics if the node doesn't exist or cannot be converted to type T
    pub fn node<T>(&self, id: NodeId<T>) -> &T
    where
        Node: TryAsRef<T>,
    {
        let node = &self.nodes[id.as_usize()];
        node.try_as_ref().unwrap()
    }

    /// Get an immutable reference to a node and its ID.
    ///
    /// # Parameters
    /// - `id`: The NodeId of the node to retrieve
    ///
    /// # Returns
    /// A tuple containing a reference to the node and its ID
    ///
    /// # Panics
    /// Panics if the node doesn't exist or cannot be converted to type T
    pub fn full<T>(&self, id: NodeId<T>) -> Full<'_, T>
    where
        Node: TryAsRef<T>,
    {
        let node = &self.nodes[id.as_usize()];
        (node.try_as_ref().unwrap(), id)
    }

    /// Get an iterator over the nodes.
    ///
    /// # Returns
    /// An iterator over all nodes in the document
    #[inline]
    pub fn iter_nodes(&self) -> std::slice::Iter<'_, Node> {
        self.nodes.iter()
    }

    /// Iterate over all nodes with their corresponding IDs.
    ///
    /// # Returns
    /// An iterator that yields each node along with its NodeId.
    /// The returned items are of type `Full<'_, Node>`, which is a tuple
    /// containing a reference to the node and its ID.
    #[inline]
    pub fn iter_full(&self) -> impl Iterator<Item = Full<'_, Node>> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(id, node)| (node, NodeId::new(id as u32)))
    }

    /// Visit the document using a visitor.
    ///
    /// This method traverses the document structure starting from the top-level blocks
    /// and calls the appropriate visitor methods for each node.
    ///
    /// # Parameters
    /// - `visitor`: The visitor implementing the Visitor trait
    ///
    /// # Returns
    /// Ok(()) if the traversal completes successfully, or an error from the visitor
    pub fn visit_by<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        for id in &self.blocks {
            visitor.visit_block(self.full(*id), self)?;
        }

        Ok(())
    }
}
