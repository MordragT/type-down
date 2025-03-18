mod block;
mod code;
mod inline;

pub use block::*;
pub use code::*;
pub use inline::*;

use derive_more::From;
use ecow::EcoString;
use thiserror::Error;

#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[from(forward)]
#[error("Faulty Node in Tree: {0}")]
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
