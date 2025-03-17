pub mod doc;
// pub mod handle;
pub mod id;
pub mod kind;
pub mod meta;
pub mod node;
pub mod tree;
pub mod visit;

pub mod prelude {
    pub use crate::Full;
    pub use crate::doc::{Doc, DocBuilder};
    // pub use crate::handle::Handler;
    pub use crate::id::NodeId;
    pub use crate::kind::NodeKind;
    pub use crate::meta::*;
    pub use crate::node::Node;
    pub use crate::tree;
    pub use crate::visit::Visitor;
}

pub type Full<'a, T> = (&'a T, id::NodeId<T>);
