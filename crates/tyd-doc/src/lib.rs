pub mod doc;
pub mod handle;
pub mod id;
pub mod kind;
pub mod meta;
pub mod node;
pub mod tree;

pub mod prelude {
    pub use crate::doc::{Doc, DocBuilder};
    pub use crate::id::NodeId;
    pub use crate::kind::NodeKind;
    pub use crate::meta::{Meta, MetaCast, Metadata, Phase};
    pub use crate::node::Node;
    pub use crate::tree;
}
