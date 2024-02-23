use crate::HtmlRender;

use super::HtmlElement;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct DynHtmlElement {
    element: HtmlElement<()>,
}

impl DynHtmlElement {
    pub fn new(tag: impl Into<String>) -> Self {
        let element = HtmlElement::<()>::init(tag);

        Self { element }
    }

    pub fn add_attribute<K, V>(&mut self, key: K, value: V) -> Option<String>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.element.add_attribute(key, value)
    }

    pub fn attribute<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.element.add_attribute(key, value);
        self
    }

    pub fn get_attribute(&self, key: impl AsRef<str>) -> Option<&String> {
        self.element.get_attribute(key)
    }

    pub fn add_class(&mut self, class: impl Into<String> + AsRef<str>) {
        self.element.add_class(class)
    }

    pub fn class(mut self, class: impl Into<String> + AsRef<str>) -> Self {
        self.element.add_class(class);
        self
    }

    pub fn add_child(&mut self, child: impl HtmlRender + 'static) {
        self.element.add_child(child);
    }

    pub fn child(mut self, child: impl HtmlRender + 'static) -> Self {
        self.element.add_child(child);
        self
    }
}

impl HtmlRender for DynHtmlElement {
    fn render(&self, rank: usize) -> String {
        self.element.render(rank)
    }
}

impl ToString for DynHtmlElement {
    fn to_string(&self) -> String {
        self.element.to_string()
    }
}

impl Into<String> for DynHtmlElement {
    fn into(self) -> String {
        self.element.into()
    }
}

impl<T> From<HtmlElement<T>> for DynHtmlElement {
    fn from(value: HtmlElement<T>) -> Self {
        let HtmlElement {
            tag,
            attributes,
            children,
            ty: _,
        } = value;

        let element = HtmlElement {
            tag,
            attributes,
            children,
            ty: PhantomData,
        };

        Self { element }
    }
}
