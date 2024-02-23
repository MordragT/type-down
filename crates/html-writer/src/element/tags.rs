use super::HtmlElement;

pub trait HrefAttr {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HtmlTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeadTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BodyTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TitleTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkTag;

impl HrefAttr for LinkTag {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreTag;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UlTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OlTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TdTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockquoteTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DelTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrongTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DivTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ATag;

impl HrefAttr for ATag {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupTag;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScriptTag;

impl<T: HrefAttr> HtmlElement<T> {
    pub fn href(self, href: impl Into<String>) -> Self {
        self.attribute("href", href)
    }
}

impl HtmlElement<HtmlTag> {
    pub fn html() -> Self {
        Self::init("html")
    }

    pub fn language(self, language: impl Into<String>) -> Self {
        self.attribute("lang", language)
    }

    pub fn set_language(&mut self, language: impl Into<String>) -> Option<String> {
        self.add_attribute("lang", language)
    }

    pub fn namespace(self, namespace: impl Into<String>) -> Self {
        self.attribute("xmlns", namespace)
    }

    pub fn set_namespace(&mut self, namespace: impl Into<String>) -> Option<String> {
        self.add_attribute("xmlns", namespace)
    }
}

impl HtmlElement<TitleTag> {
    pub fn title() -> Self {
        Self::init("title")
    }
}

impl HtmlElement<HeadTag> {
    pub fn head() -> Self {
        Self::init("head")
    }

    pub fn with_title(self, title: impl Into<String>) -> Self {
        self.child(HtmlElement::title().child(title.into()))
    }
}

impl HtmlElement<LinkTag> {
    pub fn link() -> Self {
        Self::init("link")
    }

    pub fn stylesheet(href: impl Into<String>) -> Self {
        Self::link().href(href).rel("stylesheet")
    }

    pub fn rel(self, rel: impl Into<String>) -> Self {
        self.attribute("rel", rel)
    }
}

impl HtmlElement<BodyTag> {
    pub fn body() -> Self {
        Self::init("body")
    }
}

impl HtmlElement<StyleTag> {
    pub fn style() -> Self {
        Self::init("style")
    }
}

impl HtmlElement<CodeTag> {
    pub fn code() -> Self {
        Self::init("code")
    }
}

impl HtmlElement<PreTag> {
    pub fn pre() -> Self {
        Self::init("pre")
    }
}

impl HtmlElement<UlTag> {
    pub fn ul() -> Self {
        Self::init("ul")
    }
}

impl HtmlElement<OlTag> {
    pub fn ol() -> Self {
        Self::init("ol")
    }
}

impl HtmlElement<LiTag> {
    pub fn li() -> Self {
        Self::init("li")
    }
}

impl HtmlElement<TableTag> {
    pub fn table() -> Self {
        Self::init("table")
    }
}

impl HtmlElement<TrTag> {
    pub fn tr() -> Self {
        Self::init("tr")
    }
}

impl HtmlElement<TdTag> {
    pub fn td() -> Self {
        Self::init("td")
    }
}

impl HtmlElement<BlockquoteTag> {
    pub fn blockquote() -> Self {
        Self::init("blockquote")
    }
}

impl HtmlElement<PTag> {
    pub fn p() -> Self {
        Self::init("p")
    }
}

impl HtmlElement<QTag> {
    pub fn q() -> Self {
        Self::init("q")
    }
}

impl HtmlElement<DelTag> {
    pub fn del() -> Self {
        Self::init("del")
    }
}

impl HtmlElement<StrongTag> {
    pub fn strong() -> Self {
        Self::init("strong")
    }
}

impl HtmlElement<EmTag> {
    pub fn em() -> Self {
        Self::init("em")
    }
}

impl HtmlElement<DivTag> {
    pub fn div() -> Self {
        Self::init("div")
    }
}

impl HtmlElement<ATag> {
    pub fn a() -> Self {
        Self::init("a")
    }
}

impl HtmlElement<SubTag> {
    pub fn sub() -> Self {
        Self::init("sub")
    }
}

impl HtmlElement<SupTag> {
    pub fn sup() -> Self {
        Self::init("sup")
    }
}

impl HtmlElement<ScriptTag> {
    pub fn script() -> Self {
        Self::init("script")
    }
}
