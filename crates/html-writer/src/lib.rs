use std::fmt::Debug;

pub use document::*;
pub use element::*;
pub use stack::*;

mod document;
mod element;
mod stack;

// pub const INDENT: usize = 2;
pub const TAB: &str = "  ";
pub const NAMESPACE: &str = "http://www.w3.org/1999/xhtml";
pub const DOCTYPE: &str = "<!DOCTYPE html>";
pub const LANGUAGE: &str = "en";

pub trait HtmlRender: Debug {
    fn render(&self, rank: usize) -> String;
}

impl HtmlRender for String {
    fn render(&self, rank: usize) -> String {
        // format!("{}{self}", TAB.repeat(rank))
        self.to_owned()
    }
}

impl<'a> HtmlRender for &'a str {
    fn render(&self, rank: usize) -> String {
        // format!("{}{self}", TAB.repeat(rank))
        self.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoIndent(pub String);

impl HtmlRender for NoIndent {
    fn render(&self, _: usize) -> String {
        self.0.to_owned()
    }
}

// impl<T: AsRef<str> + Debug> HtmlRender for T {
//     fn render(&self, rank: usize) -> String {
//         format!("{}{}", TAB.repeat(rank), self.as_ref())
//     }
// }
