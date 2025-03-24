/// Module containing block-level elements for document structure
mod block;
/// Module containing code-related elements and parsing utilities
mod code;
/// Module containing inline text elements and formatting
mod inline;

pub use block::*;
pub use code::*;
pub use inline::*;

use derive_more::From;
use ecow::EcoString;
use thiserror::Error;

/// Represents an error that occurs during document tree parsing or manipulation
///
/// Wraps a string message that describes the specific error condition.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[from(forward)]
#[error("Faulty Node in Tree: {0}")]
pub struct Error(pub EcoString);

/// Represents an tag in the document
///
/// Contains the string representation of the tag name.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Tag(pub EcoString);

/// Represents plain text content in the document
///
/// Stores the actual text string that appears in the document.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Text(pub EcoString);

/// Represents a label used for references, links, or other identifiable elements
///
/// Contains the string value of the label.
#[derive(Debug, From, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[from(forward)]
pub struct Label(pub EcoString);
