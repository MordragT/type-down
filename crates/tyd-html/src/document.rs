use crate::{
    element::{tags::HtmlTag, HtmlElement, HtmlRender},
    DOCTYPE, LANGUAGE, NAMESPACE,
};

#[derive(Debug)]
pub struct HtmlDocument {
    doctype: Option<String>,
    root: HtmlElement<HtmlTag>,
}

impl Default for HtmlDocument {
    fn default() -> Self {
        Self::new()
            .default_doctype()
            .default_language()
            .default_namespace()
    }
}

impl HtmlDocument {
    pub fn new() -> Self {
        Self {
            doctype: None,
            root: HtmlElement::html(),
        }
    }

    pub fn default_doctype(self) -> Self {
        self.doctype(DOCTYPE)
    }

    pub fn doctype(mut self, doctype: impl Into<String>) -> Self {
        self.set_doctype(doctype);
        self
    }

    pub fn set_doctype(&mut self, doctype: impl Into<String>) {
        self.doctype = Some(doctype.into());
    }

    pub fn default_language(mut self) -> Self {
        self.root.set_language(LANGUAGE);
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.root.set_language(language);
        self
    }

    pub fn set_language(&mut self, language: impl Into<String>) -> Option<String> {
        self.root.set_language(language)
    }

    pub fn default_namespace(mut self) -> Self {
        self.root.set_namespace(NAMESPACE);
        self
    }

    pub fn namespace(mut self, namespace: impl Into<String>) -> Self {
        self.root.set_namespace(namespace);
        self
    }

    pub fn set_namespace(&mut self, namespace: impl Into<String>) -> Option<String> {
        self.root.set_namespace(namespace)
    }

    pub fn add_child(&mut self, child: impl HtmlRender + 'static + 'static) {
        self.root.add_child(child);
    }

    pub fn child(mut self, child: impl HtmlRender + 'static) -> Self {
        self.root.add_child(child);
        self
    }
}

impl ToString for HtmlDocument {
    fn to_string(&self) -> String {
        let Self { doctype, root } = self;

        let mut buffer = String::new();

        if let Some(doctype) = doctype {
            buffer.push_str(doctype);
            buffer.push('\n');
        }

        let contents = root.to_string();
        buffer.push_str(&contents);

        buffer
    }
}

impl Into<String> for HtmlDocument {
    fn into(self) -> String {
        let Self { doctype, root } = self;

        let mut buffer = match doctype {
            Some(mut doctype) => {
                doctype.push('\n');
                doctype
            }
            None => String::new(),
        };

        let contents: String = root.into();
        buffer.push_str(&contents);

        buffer
    }
}
