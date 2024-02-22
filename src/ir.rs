use enum_map::{Enum, EnumMap};

pub struct Document {
    pub blocks: Vec<Block>,
}

// pub struct Block {
//     kind: BlockKind,
//     label: Option<String>,
// }

// pub enum BlockKind {}

pub struct Inline {
    kind: InlineKind,
    // attrs: Attributes,
}

pub enum InlineKind {}

// pub type Attributes = EnumMap<AttributeKind, String>;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Enum)]
// pub enum AttributeKind {
//     Label,
//     Custom(u8),
// }
