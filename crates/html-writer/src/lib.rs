pub use document::*;
pub use element::*;
pub use stack::*;

mod document;
mod element;
mod stack;

pub const INDENT: usize = 2;
pub const NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
pub const DOCTYPE: &str = "<!DOCTYPE html>";
pub const LANGUAGE: &str = "en";
