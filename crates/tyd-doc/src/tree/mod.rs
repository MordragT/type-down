mod block;
mod code;
mod inline;

pub use block::*;
pub use code::*;
pub use inline::*;

use derive_more::From;
use ecow::EcoString;

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Error(pub EcoString);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Tag(pub EcoString);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Text(pub EcoString);

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Label(pub EcoString);
