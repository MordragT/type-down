// TODO font-family etc.

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Context {
    pub title: String,
    pub source: PathBuf,
    pub dest: PathBuf,
}

impl Context {
    pub fn new(title: String, source: PathBuf, dest: PathBuf) -> Self {
        Self {
            title,
            source,
            dest,
        }
    }
}
