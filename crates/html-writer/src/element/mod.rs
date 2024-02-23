use crate::INDENT;
use std::{collections::HashMap, marker::PhantomData};

pub use dynamic::*;

mod dynamic;
pub mod tags;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlElement<T> {
    tag: String,
    attributes: HashMap<String, String>,
    children: Vec<String>,
    ty: PhantomData<T>,
}

impl<T> HtmlElement<T> {
    fn init(tag: impl Into<String>) -> Self {
        let tag = tag.into();

        Self {
            tag,
            attributes: HashMap::new(),
            children: Vec::new(),
            ty: PhantomData,
        }
    }

    pub fn add_attribute<K, V>(&mut self, key: K, value: V) -> Option<String>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.attributes.insert(key.into(), value.into())
    }

    pub fn attribute<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.add_attribute(key, value);
        self
    }

    pub fn get_attribute(&self, key: impl AsRef<str>) -> Option<&String> {
        self.attributes.get(key.as_ref())
    }

    pub fn add_class(&mut self, class: impl Into<String> + AsRef<str>) {
        if let Some(classes) = self.attributes.get_mut("class") {
            classes.push(' ');
            classes.push_str(class.as_ref());
        } else {
            self.add_attribute("class", class.into());
        }
    }

    pub fn class(mut self, class: impl Into<String> + AsRef<str>) -> Self {
        self.add_class(class);
        self
    }

    pub fn add_child(&mut self, child: impl Into<String>) {
        self.children.push(child.into())
    }

    pub fn child(mut self, child: impl Into<String>) -> Self {
        self.add_child(child);
        self
    }
}

impl<T> ToString for HtmlElement<T> {
    fn to_string(&self) -> String {
        let Self {
            tag,
            attributes,
            children,
            ty: _,
        } = &self;

        let mut buffer = String::new();

        let attrs = attributes
            .iter()
            .map(|(key, value)| format!("{key}=\"{value}\""))
            .collect::<Vec<_>>();
        let attrs = attrs.join(" ");

        let open = format!("<{tag} {attrs}>");
        buffer.push_str(&open);

        let indentation = " ".repeat(INDENT);
        let separator = format!("\n{}", &indentation);
        let children = children.join(&separator);

        let mut body = separator;
        body.push_str(&children);
        buffer.push_str(&body);

        let close = format!("</{tag}>");
        buffer.push_str(&close);

        buffer
    }
}

impl<T> Into<String> for HtmlElement<T> {
    fn into(self) -> String {
        self.to_string()
    }
}
