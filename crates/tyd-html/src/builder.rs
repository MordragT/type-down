// use html_writer::{
//     tags::{BodyTag, HeadTag},
//     DynHtmlElement, HtmlDocument, HtmlElement, HtmlStack, NoIndent,
// };

// use self::visitor::{
//     walk_block_quote, walk_emphasis, walk_enclosed, walk_heading, walk_paragraph, walk_quote,
//     walk_strikeout, walk_strong, walk_table, Visitor,
// };

// use super::{Compiler, Context, Output};
// use crate::parse::ast::*;

use tyd_render::{Args, Context, Value};
use tyd_syntax::ast::{
    visitor::{
        walk_block_quote, walk_emphasis, walk_enclosed, walk_heading, walk_paragraph, walk_quote,
        walk_strikeout, walk_strong, walk_table, Visitor,
    },
    *,
};

use crate::{
    document::HtmlDocument,
    element::{
        tags::{BodyTag, HeadTag},
        DynHtmlElement, HtmlElement, NoIndent,
    },
    stack::HtmlStack,
    HtmlError,
};

// #[derive(Debug)]
pub struct HtmlBuilder {
    head: HtmlElement<HeadTag>,
    body: HtmlElement<BodyTag>,
    stack: HtmlStack,
    ctx: Context,
}

impl HtmlBuilder {
    pub fn new(ctx: Context) -> Self {
        let head = HtmlElement::head().child(HtmlElement::stylesheet(
            "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css",
        ));
        // .with_title(&ctx.title);

        Self {
            head,
            body: HtmlElement::body(),
            stack: HtmlStack::new(),
            ctx,
        }
    }

    pub fn build(self) -> HtmlDocument {
        let Self {
            head,
            body,
            stack,
            ctx: _,
        } = self;

        assert!(stack.is_empty());

        let body = body
            .child(HtmlElement::script().attribute(
                "src",
                "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js",
            ))
            .child(HtmlElement::script().attribute(
                "src",
                "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/go.min.js",
            ))
            .child(HtmlElement::script().child("hljs.highlightAll();"));

        HtmlDocument::new()
            .default_doctype()
            .default_language()
            .default_namespace()
            .child(head)
            .child(body)
    }
}

impl Visitor for HtmlBuilder {
    type Error = HtmlError;

    fn visit_raw(&mut self, raw: &Raw) -> Result<(), Self::Error> {
        let mut code = HtmlElement::code().child(NoIndent(raw.content.to_owned()));

        if let Some(lang) = &raw.lang {
            code.add_class(format!("language-{lang}"));
        }

        let pre = HtmlElement::pre().child(code);
        self.body.add_child(pre);

        Ok(())
    }

    fn visit_heading(&mut self, heading: &Heading) -> Result<(), Self::Error> {
        let h = DynHtmlElement::new(&format!("h{}", heading.level));
        let pos = self.stack.start(h);

        walk_heading(self, heading)?;

        let h = self.stack.end(pos);
        self.body.add_child(h);

        Ok(())
    }

    fn visit_bullet_list(&mut self, list: &BulletList) -> Result<(), Self::Error> {
        let pos = self.stack.start(HtmlElement::ul());

        for line in &list.lines {
            let pos = self.stack.start(HtmlElement::li());

            self.visit_line(line)?;

            self.stack.fold(pos);
        }

        let ul = self.stack.end(pos);
        self.body.add_child(ul);

        Ok(())
    }

    fn visit_ordered_list(&mut self, ordered_list: &OrderedList) -> Result<(), Self::Error> {
        let pos = self.stack.start(HtmlElement::ol());

        for line in &ordered_list.lines {
            let pos = self.stack.start(HtmlElement::li());

            self.visit_line(line)?;

            self.stack.fold(pos);
        }

        let ol = self.stack.end(pos);
        self.body.add_child(ol);

        Ok(())
    }

    fn visit_table(&mut self, table: &Table) -> Result<(), Self::Error> {
        let pos = self.stack.start(HtmlElement::table());

        walk_table(self, table)?;

        let tbl = self.stack.end(pos);
        self.body.add_child(tbl);

        Ok(())
    }

    fn visit_table_row(&mut self, table_row: &TableRow) -> Result<(), Self::Error> {
        let pos = self.stack.start(HtmlElement::tr());

        for cell in &table_row.cells {
            let pos = self.stack.start(HtmlElement::td());

            self.visit_elements(cell)?;

            self.stack.fold(pos);
        }

        self.stack.fold(pos);

        Ok(())
    }

    fn visit_block_quote(&mut self, block_quote: &BlockQuote) -> Result<(), Self::Error> {
        let pos = self.stack.start(HtmlElement::blockquote());

        walk_block_quote(self, block_quote)?;

        let blqt = self.stack.end(pos);
        self.body.add_child(blqt);

        Ok(())
    }

    fn visit_paragraph(&mut self, paragraph: &Paragraph) -> Result<(), Self::Error> {
        let pos = self.stack.start(HtmlElement::p());

        walk_paragraph(self, paragraph)?;

        let p = self.stack.end(pos);
        self.body.add_child(p);

        Ok(())
    }

    fn visit_quote(&mut self, quote: &Quote) -> Result<(), Self::Error> {
        let pos = self.stack.start(HtmlElement::q());

        walk_quote(self, quote)?;

        self.stack.fold(pos);
        Ok(())
    }

    fn visit_strikeout(&mut self, strikeout: &Strikeout) -> Result<(), Self::Error> {
        let pos = self.stack.start(HtmlElement::del());

        walk_strikeout(self, strikeout)?;

        self.stack.fold(pos);
        Ok(())
    }

    fn visit_strong(&mut self, strong: &Strong) -> Result<(), Self::Error> {
        let pos = self.stack.start(HtmlElement::strong());

        walk_strong(self, strong)?;

        self.stack.fold(pos);
        Ok(())
    }

    fn visit_emphasis(&mut self, emphasis: &Emphasis) -> Result<(), Self::Error> {
        let pos = self.stack.start(HtmlElement::em());

        walk_emphasis(self, emphasis)?;

        self.stack.fold(pos);
        Ok(())
    }

    fn visit_enclosed(&mut self, enclosed: &Content) -> Result<(), Self::Error> {
        let pos = self.stack.start(HtmlElement::div());

        walk_enclosed(self, enclosed)?;

        self.stack.fold(pos);
        Ok(())
    }

    fn visit_link(&mut self, link: &Link) -> Result<(), Self::Error> {
        let a = HtmlElement::a().href(&link.link);

        if let Some(elements) = &link.elements {
            let pos = self.stack.start(a);

            self.visit_elements(elements)?;

            self.stack.fold(pos);
        } else {
            self.stack.add_child(a.child(link.link.to_owned()));
        }

        Ok(())
    }

    fn visit_escape(&mut self, escape: &Escape) -> Result<(), Self::Error> {
        self.stack.add_child(escape.0.to_string());
        Ok(())
    }

    fn visit_raw_inline(&mut self, raw_inline: &RawInline) -> Result<(), Self::Error> {
        self.stack
            .add_child(HtmlElement::code().child(raw_inline.0.to_owned()));
        Ok(())
    }

    fn visit_sub_script(&mut self, sub_script: &SubScript) -> Result<(), Self::Error> {
        self.stack
            .add_child(HtmlElement::sub().child(sub_script.0.to_string()));
        Ok(())
    }

    fn visit_sup_script(&mut self, sup_script: &SupScript) -> Result<(), Self::Error> {
        self.stack
            .add_child(HtmlElement::sup().child(sup_script.0.to_string()));
        Ok(())
    }

    fn visit_spacing(&mut self, spacing: &Spacing) -> Result<(), Self::Error> {
        self.stack.add_child(" ".repeat(spacing.0));
        Ok(())
    }

    fn visit_word(&mut self, word: &Word) -> Result<(), Self::Error> {
        self.stack.add_child(word.0.to_owned());
        Ok(())
    }

    fn visit_access(&mut self, access: &Access) -> Result<(), Self::Error> {
        let Access { ident, tail } = access;

        if let Some(CallTail { args, content }) = tail {
            // FIXME better argument handling and evaluation layer
            let mut f_args = Args::new();

            for (key, value) in args {
                match value {
                    Value::Identifier(ident) => {
                        let object = self.ctx.get(ident).unwrap();
                        f_args.insert(key.clone(), object.clone());
                    }
                    Value::String(s) => {
                        f_args.insert(key.clone(), Value::Str(s.clone()));
                    }
                }
            }

            let f = self.ctx.call(ident).unwrap();
            let result = f(f_args)?;

            match result {
                Value::Block(block) => self.visit_block(&block)?,
                Value::Element(el) => self.visit_element(&el)?,
                // TODO error
                _ => panic!("access calls must return block or element"),
            }

            // let pos = self.stack.start(f(args));

            // if let Some(enclosed) = content {
            //     self.visit_enclosed(enclosed);
            // }

            // self.stack.fold(pos);
        } else {
            let object = self.ctx.get(ident).unwrap().clone();

            match object {
                Value::Block(block) => self.visit_block(&block)?,
                Value::Element(el) => self.visit_element(&el)?,
                // TODO error
                _ => panic!("access calls must return block or element"),
            }
        }
        Ok(())
    }
}
