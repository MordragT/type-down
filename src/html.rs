use crate::{ast::*, context::Context, Compiler};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HtmlCompiler;

impl Compiler for HtmlCompiler {
    type Error = io::Error;

    fn compile(ctx: &Context, ast: &Ast) -> Result<(), Self::Error> {
        let title = &ctx.title;
        let body = ast.to_html().to_string();

        let highlight = "<link rel=\"stylesheet\" href=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css\"><script src=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js\"></script><script src=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/go.min.js\"></script><script>hljs.highlightAll();</script>";
        let html = format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">{highlight}<title>{title}</title></head>{body}</html>");
        std::fs::write(&ctx.dest, html)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HtmlElement {
    tag: Option<String>,
    class: Option<String>,
    href: Option<String>,
    body: String,
    next: Option<Box<Self>>,
}

impl HtmlElement {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: Some(tag.to_owned()),
            body: String::new(),
            class: None,
            href: None,
            next: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            tag: None,
            body: String::new(),
            class: None,
            href: None,
            next: None,
        }
    }

    pub fn body(body: &str) -> Self {
        Self {
            tag: None,
            body: body.to_owned(),
            class: None,
            href: None,
            next: None,
        }
    }

    pub fn push(&mut self, element: &str) {
        self.body.push_str(element);
    }

    pub fn set_class(&mut self, class: String) {
        self.class = Some(class);
    }

    pub fn with(mut self, element: &str) -> Self {
        self.body.push_str(element);
        self
    }

    pub fn with_next(mut self, element: HtmlElement) -> Self {
        self.next = Some(Box::new(element));
        self
    }

    pub fn with_class(mut self, class: String) -> Self {
        self.class = Some(class);
        self
    }

    pub fn with_href(mut self, href: String) -> Self {
        self.href = Some(href);
        self
    }
}

impl ToString for HtmlElement {
    fn to_string(&self) -> String {
        let Self {
            tag,
            body,
            next,
            class,
            href,
        } = self;

        let mut this = match tag {
            Some(tag) => {
                let mut left = tag.to_owned();

                if let Some(class) = class {
                    left.push_str(&format!(" class=\"{class}\""));
                }

                if let Some(href) = href {
                    left.push_str(&format!(" href=\"{href}\""));
                }

                format!("<{left}>{body}</{tag}>")
            }
            None => body.to_owned(),
        };

        match next {
            Some(next) => this.push_str(&next.to_string()),
            None => (),
        }

        this
    }
}

pub trait ToHtml {
    fn to_html(&self) -> HtmlElement;
}

impl ToHtml for Ast {
    fn to_html(&self) -> HtmlElement {
        let mut body = HtmlElement::new("body");

        for block in &self.blocks {
            body.push(&block.to_html().to_string());
        }

        body
    }
}

impl ToHtml for Block {
    fn to_html(&self) -> HtmlElement {
        match self {
            Self::Raw(raw) => raw.to_html(),
            Self::Mark(block) => block.to_html(),
        }
    }
}

impl ToHtml for RawBlock {
    fn to_html(&self) -> HtmlElement {
        let mut code = HtmlElement::new("code").with(&self.content);

        if let Some(lang) = &self.lang {
            code.set_class(format!("language-{lang}"));
        }

        HtmlElement::new("pre").with(&code.to_string())
    }
}

impl ToHtml for MarkBlock {
    fn to_html(&self) -> HtmlElement {
        match self {
            Self::Heading(heading) => heading.to_html(),
            Self::List(list) => list.to_html(),
            Self::OrderedList(ordered) => ordered.to_html(),
            Self::Table(table) => table.to_html(),
            Self::Blockquote(blockquote) => blockquote.to_html(),
            Self::Paragraph(paragraph) => paragraph.to_html(),
        }
    }
}

impl ToHtml for Heading {
    fn to_html(&self) -> HtmlElement {
        HtmlElement::new(&format!("h{}", self.level)).with(&self.line.to_html().to_string())
    }
}

impl ToHtml for Paragraph {
    fn to_html(&self) -> HtmlElement {
        let mut p = HtmlElement::new("p");

        for line in &self.lines {
            p.push(&line.to_html().to_string());
        }

        p
    }
}

impl ToHtml for List {
    fn to_html(&self) -> HtmlElement {
        let mut ul = HtmlElement::new("ul");

        for line in &self.lines {
            let li = HtmlElement::new("li").with(&line.to_html().to_string());
            ul.push(&li.to_string());
        }

        ul
    }
}

impl ToHtml for OrderedList {
    fn to_html(&self) -> HtmlElement {
        let mut ol = HtmlElement::new("ol");

        for line in &self.lines {
            let li = HtmlElement::new("li").with(&line.to_html().to_string());
            ol.push(&li.to_string());
        }

        ol
    }
}

impl ToHtml for Table {
    fn to_html(&self) -> HtmlElement {
        let mut table = HtmlElement::new("table");

        for row in &self.rows {
            let tr = row.to_html();
            table.push(&tr.to_string());
        }

        table
    }
}

impl ToHtml for TableRow {
    fn to_html(&self) -> HtmlElement {
        let mut tr = HtmlElement::new("tr");

        for el in &self.elements {
            let td = HtmlElement::new("td").with(&el.to_html().to_string());
            tr.push(&td.to_string());
        }

        tr
    }
}

impl ToHtml for Blockquote {
    fn to_html(&self) -> HtmlElement {
        let mut blockquote = HtmlElement::new("blockquote");

        for line in &self.lines {
            blockquote.push(&line.to_html().to_string());
        }

        blockquote
    }
}

impl ToHtml for Line {
    fn to_html(&self) -> HtmlElement {
        self.elements.to_html()
    }
}

impl ToHtml for Elements {
    fn to_html(&self) -> HtmlElement {
        let mut empty = HtmlElement::empty();

        for el in &self.0 {
            empty.push(&el.to_html().to_string());
            empty.push(" ");
        }

        empty
    }
}

impl ToHtml for Element {
    fn to_html(&self) -> HtmlElement {
        match self {
            Self::Quote(quote) => quote.to_html(),
            Self::Strikethrough(strike) => strike.to_html(),
            Self::Emphasis(emphasis) => emphasis.to_html(),
            Self::Strong(strong) => strong.to_html(),
            Self::Enclosed(enclosed) => enclosed.to_html(),
            Self::Link(link) => link.to_html(),
            Self::Escape(escape) => escape.to_html(),
            Self::Monospace(monospace) => monospace.to_html(),
            Self::Script(script) => script.to_html(),
        }
    }
}

impl ToHtml for Quote {
    fn to_html(&self) -> HtmlElement {
        let mut q = HtmlElement::new("q");

        for el in &self.elements {
            q.push(&el.to_html().to_string());
            q.push(" ");
        }

        q
    }
}

impl ToHtml for QuoteElement {
    fn to_html(&self) -> HtmlElement {
        match self {
            Self::Strikethrough(strike) => strike.to_html(),
            Self::Emphasis(emphasis) => emphasis.to_html(),
            Self::Strong(strong) => strong.to_html(),
            Self::Script(script) => script.to_html(),
        }
    }
}

impl ToHtml for Strikethrough {
    fn to_html(&self) -> HtmlElement {
        let mut del = HtmlElement::new("del");

        for el in &self.elements {
            del.push(&el.to_html().to_string());
            del.push(" ");
        }

        del
    }
}

impl ToHtml for StrikethroughElement {
    fn to_html(&self) -> HtmlElement {
        match self {
            Self::Emphasis(emphasis) => emphasis.to_html(),
            Self::Strong(strong) => strong.to_html(),
            Self::Script(script) => script.to_html(),
        }
    }
}

impl ToHtml for Emphasis {
    fn to_html(&self) -> HtmlElement {
        let mut em = HtmlElement::new("em");

        for script in &self.scripts {
            em.push(&script.to_html().to_string());
            em.push(" ");
        }

        em
    }
}

impl ToHtml for Strong {
    fn to_html(&self) -> HtmlElement {
        let mut strong = HtmlElement::new("strong");

        for script in &self.scripts {
            strong.push(&script.to_html().to_string());
            strong.push(" ");
        }

        strong
    }
}

impl ToHtml for Enclosed {
    fn to_html(&self) -> HtmlElement {
        HtmlElement::new("div").with(&self.elements.to_html().to_string())
    }
}

impl ToHtml for Link {
    fn to_html(&self) -> HtmlElement {
        let display = match &self.elements {
            Some(elements) => elements.to_html().to_string(),
            None => self.link.clone(),
        };

        HtmlElement::new("a")
            .with_href(self.link.clone())
            .with(&display)
    }
}

impl ToHtml for Escape {
    fn to_html(&self) -> HtmlElement {
        // TODO escape html style
        HtmlElement::body(&self.0.escape_default().to_string())
    }
}

impl ToHtml for Monospace {
    fn to_html(&self) -> HtmlElement {
        HtmlElement::new("code").with(&self.0)
    }
}

impl ToHtml for Script {
    fn to_html(&self) -> HtmlElement {
        HtmlElement::body(&self.0).with(&self.1.to_html().to_string())
    }
}

impl ToHtml for ScriptTail {
    fn to_html(&self) -> HtmlElement {
        match self {
            ScriptTail::Sup(c, script) => HtmlElement::new("sup")
                .with(&c.to_string())
                .with_next(script.to_html()),
            ScriptTail::Sub(c, script) => HtmlElement::new("sub")
                .with(&c.to_string())
                .with_next(script.to_html()),
            ScriptTail::None => HtmlElement::empty(),
        }
    }
}
