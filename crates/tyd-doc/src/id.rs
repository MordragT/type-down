use std::{hash::Hash, marker::PhantomData};

#[derive(Debug)]
pub struct NodeId<T> {
    id: u32,
    t: PhantomData<T>,
}

impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            t: PhantomData,
        }
    }
}

impl<T> Copy for NodeId<T> {}

impl<T> PartialEq for NodeId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<T> Eq for NodeId<T> {}

impl<T> PartialOrd for NodeId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T> Ord for NodeId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T> Hash for NodeId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> NodeId<T> {
    pub(crate) fn from_usize(id: usize) -> Self {
        Self {
            id: id as u32,
            t: PhantomData,
        }
    }

    pub(crate) fn new(id: u32) -> Self {
        Self { id, t: PhantomData }
    }

    pub fn as_usize(&self) -> usize {
        self.id as usize
    }

    // pub fn get(self, tree: &impl NodeContainer) -> &T
    // where
    //     T: InnerNode,
    // {
    //     tree.node(self)
    // }

    // pub fn meta<P>(self, metadata: &impl MetaContainer<P>) -> &T::Meta
    // where
    //     P: Phase,
    //     T: Attached<P>,
    // {
    //     metadata.meta(self)
    // }

    // pub fn meta_mut<P, M>(self, metadata: &mut impl MetaContainer<P>) -> &mut T::Meta
    // where
    //     P: Phase,
    //     T: Attached<P>,
    // {
    //     metadata.meta_mut(self)
    // }

    // pub(crate) fn cast<U>(self) -> NodeId<U> {
    //     NodeId {
    //         id: self.id,
    //         t: PhantomData,
    //     }
    // }
}
