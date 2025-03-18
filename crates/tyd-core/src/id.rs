use std::{hash::Hash, marker::PhantomData};

use crate::meta::{MetaCast, MetaContainer, Phase};

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
    pub(crate) fn new(id: u32) -> Self {
        Self { id, t: PhantomData }
    }

    pub fn as_usize(&self) -> usize {
        self.id as usize
    }

    pub fn meta<P>(self, metadata: &impl MetaContainer<P>) -> &T::Meta
    where
        P: Phase,
        T: MetaCast<P>,
    {
        metadata.meta(self)
    }
}
