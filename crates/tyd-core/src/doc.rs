use std::sync::Arc;

use crate::{Full, TryAsMut, TryAsRef, id::NodeId, node::Node, tree::Block, visit::Visitor};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DocBuilder {
    nodes: Vec<Node>,
}

impl DocBuilder {
    pub fn node<T>(&self, id: NodeId<T>) -> &T
    where
        Node: TryAsRef<T>,
    {
        let node = &self.nodes[id.as_usize()];
        node.try_as_ref().unwrap()
    }

    pub fn node_mut<T>(&mut self, id: NodeId<T>) -> &mut T
    where
        Node: TryAsMut<T>,
    {
        let node = &mut self.nodes[id.as_usize()];
        node.try_as_mut().unwrap()
    }

    pub fn update_node<T>(&mut self, id: NodeId<T>, node: T) -> T
    where
        Node: TryAsMut<T>,
    {
        std::mem::replace(self.node_mut(id), node)
    }

    pub fn insert<T>(&mut self, node: T) -> NodeId<T>
    where
        Node: From<T>,
    {
        let id = self.nodes.len() as u32;

        self.nodes.push(Node::from(node));

        NodeId::new(id)
    }

    pub fn finish(self, blocks: Vec<NodeId<Block>>) -> Doc {
        let Self { nodes } = self;

        let nodes = Arc::new(nodes);

        Doc { blocks, nodes }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Doc {
    blocks: Vec<NodeId<Block>>,
    nodes: Arc<Vec<Node>>,
}

impl Doc {
    #[inline]
    pub fn blocks(&self) -> &Vec<NodeId<Block>> {
        &self.blocks
    }

    #[inline]
    pub fn get(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    pub fn node<T>(&self, id: NodeId<T>) -> &T
    where
        Node: TryAsRef<T>,
    {
        let node = &self.nodes[id.as_usize()];
        node.try_as_ref().unwrap()
    }

    pub fn full<T>(&self, id: NodeId<T>) -> Full<'_, T>
    where
        Node: TryAsRef<T>,
    {
        let node = &self.nodes[id.as_usize()];
        (node.try_as_ref().unwrap(), id)
    }

    #[inline]
    pub fn iter_nodes(&self) -> std::slice::Iter<'_, Node> {
        self.nodes.iter()
    }

    pub fn visit_by<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
        for id in &self.blocks {
            visitor.visit_block(self.full(*id), self)?;
        }

        Ok(())
    }
}
