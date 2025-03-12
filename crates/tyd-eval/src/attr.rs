use crate::ir::Attr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttrBuilder {
    ident: String,
    classes: Vec<String>,
    pairs: Vec<(String, String)>,
}

impl AttrBuilder {
    pub fn new() -> Self {
        Self {
            ident: String::new(),
            classes: Vec::new(),
            pairs: Vec::new(),
        }
    }

    pub fn ident(mut self, ident: impl Into<String>) -> Self {
        self.ident = ident.into();
        self
    }

    pub fn ident_opt(mut self, ident: Option<impl Into<String>>) -> Self {
        if let Some(ident) = ident {
            self.ident = ident.into();
        }
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    pub fn class_opt(mut self, class: Option<impl Into<String>>) -> Self {
        if let Some(class) = class {
            self.classes.push(class.into())
        }
        self
    }

    pub fn attr<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.pairs.push((key.into(), value.into()));
        self
    }

    pub fn add_attr<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.pairs.push((key.into(), value.into()));
    }

    pub fn build(self) -> Attr {
        let Self {
            ident,
            classes,
            pairs,
        } = self;
        (ident, classes, pairs)
    }

    pub fn empty() -> Attr {
        (String::new(), Vec::new(), Vec::new())
    }
}
