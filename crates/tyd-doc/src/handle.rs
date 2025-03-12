use crate::{id::NodeId, node::Node, tree};

pub trait Handler {
    type Error;

    fn handle_node(&mut self, node: &Node, id: usize) -> Result<(), Self::Error> {
        use Node::*;

        match node {
            Error(error) => self.handle_error(error, NodeId::from_usize(id)),
            Tag(tag) => self.handle_tag(tag, NodeId::from_usize(id)),
            Text(text) => self.handle_text(text, NodeId::from_usize(id)),
            Label(label) => self.handle_label(label, NodeId::from_usize(id)),

            // Block
            Block(block) => self.handle_block(block, NodeId::from_usize(id)),
            Raw(raw) => self.handle_raw(raw, NodeId::from_usize(id)),
            Heading(heading) => self.handle_heading(heading, NodeId::from_usize(id)),
            HeadingMarker(heading_marker) => {
                self.handle_heading_marker(heading_marker, NodeId::from_usize(id))
            }
            Table(table) => self.handle_table(table, NodeId::from_usize(id)),
            TableRow(table_row) => self.handle_table_row(table_row, NodeId::from_usize(id)),
            List(list) => self.handle_list(list, NodeId::from_usize(id)),
            ListItem(list_item) => self.handle_list_item(list_item, NodeId::from_usize(id)),
            Enum(enum_val) => self.handle_enum(enum_val, NodeId::from_usize(id)),
            EnumItem(enum_item) => self.handle_enum_item(enum_item, NodeId::from_usize(id)),
            Terms(terms) => self.handle_terms(terms, NodeId::from_usize(id)),
            TermItem(term_item) => self.handle_term_item(term_item, NodeId::from_usize(id)),
            Paragraph(paragraph) => self.handle_paragraph(paragraph, NodeId::from_usize(id)),
            Plain(plain) => self.handle_plain(plain, NodeId::from_usize(id)),

            // Inline
            Inline(inline) => self.handle_inline(inline, NodeId::from_usize(id)),
            Quote(quote) => self.handle_quote(quote, NodeId::from_usize(id)),
            Strikeout(strikeout) => self.handle_strikeout(strikeout, NodeId::from_usize(id)),
            Emphasis(emphasis) => self.handle_emphasis(emphasis, NodeId::from_usize(id)),
            Strong(strong) => self.handle_strong(strong, NodeId::from_usize(id)),
            Subscript(subscript) => self.handle_subscript(subscript, NodeId::from_usize(id)),
            Supscript(supscript) => self.handle_supscript(supscript, NodeId::from_usize(id)),
            Link(link) => self.handle_link(link, NodeId::from_usize(id)),
            Ref(ref_val) => self.handle_ref(ref_val, NodeId::from_usize(id)),
            RawInline(raw_inline) => self.handle_raw_inline(raw_inline, NodeId::from_usize(id)),
            MathInline(math_inline) => self.handle_math_inline(math_inline, NodeId::from_usize(id)),
            Comment(comment) => self.handle_comment(comment, NodeId::from_usize(id)),
            Escape(escape) => self.handle_escape(escape, NodeId::from_usize(id)),
            Word(word) => self.handle_word(word, NodeId::from_usize(id)),
            Spacing(spacing) => self.handle_spacing(spacing, NodeId::from_usize(id)),
            SoftBreak(soft_break) => self.handle_soft_break(soft_break, NodeId::from_usize(id)),

            // Code
            Code(code) => self.handle_code(code, NodeId::from_usize(id)),
            Expr(expr) => self.handle_expr(expr, NodeId::from_usize(id)),
            ExprBlock(expr_block) => self.handle_expr_block(expr_block, NodeId::from_usize(id)),
            Ident(ident) => self.handle_ident(ident, NodeId::from_usize(id)),
            Call(call) => self.handle_call(call, NodeId::from_usize(id)),
            Args(args) => self.handle_args(args, NodeId::from_usize(id)),
            Arg(arg) => self.handle_arg(arg, NodeId::from_usize(id)),
            Literal(literal) => self.handle_literal(literal, NodeId::from_usize(id)),
            Content(content) => self.handle_content(content, NodeId::from_usize(id)),
        }
    }

    fn handle_error(
        &mut self,
        _error: &tree::Error,
        _id: NodeId<tree::Error>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_tag(&mut self, _tag: &tree::Tag, _id: NodeId<tree::Tag>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_text(
        &mut self,
        _text: &tree::Text,
        _id: NodeId<tree::Text>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_label(
        &mut self,
        _label: &tree::Label,
        _id: NodeId<tree::Label>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_block(
        &mut self,
        _block: &tree::Block,
        _id: NodeId<tree::Block>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_raw(&mut self, _raw: &tree::Raw, _id: NodeId<tree::Raw>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_heading(
        &mut self,
        _heading: &tree::Heading,
        _id: NodeId<tree::Heading>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_heading_marker(
        &mut self,
        _heading_marker: &tree::HeadingMarker,
        _id: NodeId<tree::HeadingMarker>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_table(
        &mut self,
        _table: &tree::Table,
        _id: NodeId<tree::Table>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_table_row(
        &mut self,
        _table_row: &tree::TableRow,
        _id: NodeId<tree::TableRow>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_list(
        &mut self,
        _list: &tree::List,
        _id: NodeId<tree::List>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_list_item(
        &mut self,
        _list_item: &tree::ListItem,
        _id: NodeId<tree::ListItem>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_enum(
        &mut self,
        _enumeration: &tree::Enum,
        _id: NodeId<tree::Enum>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_enum_item(
        &mut self,
        _enum_item: &tree::EnumItem,
        _id: NodeId<tree::EnumItem>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_terms(
        &mut self,
        _terms: &tree::Terms,
        _id: NodeId<tree::Terms>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_term_item(
        &mut self,
        _term_item: &tree::TermItem,
        _id: NodeId<tree::TermItem>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_paragraph(
        &mut self,
        _paragraph: &tree::Paragraph,
        _id: NodeId<tree::Paragraph>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_plain(
        &mut self,
        _plain: &tree::Plain,
        _id: NodeId<tree::Plain>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_inline(
        &mut self,
        _inline: &tree::Inline,
        _id: NodeId<tree::Inline>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_quote(
        &mut self,
        _quote: &tree::Quote,
        _id: NodeId<tree::Quote>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_strikeout(
        &mut self,
        _strikeout: &tree::Strikeout,
        _id: NodeId<tree::Strikeout>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_emphasis(
        &mut self,
        _emphasis: &tree::Emphasis,
        _id: NodeId<tree::Emphasis>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_strong(
        &mut self,
        _strong: &tree::Strong,
        _id: NodeId<tree::Strong>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_subscript(
        &mut self,
        _subscript: &tree::Subscript,
        _id: NodeId<tree::Subscript>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_supscript(
        &mut self,
        _supscript: &tree::Supscript,
        _id: NodeId<tree::Supscript>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_link(
        &mut self,
        _link: &tree::Link,
        _id: NodeId<tree::Link>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_ref(
        &mut self,
        _reference: &tree::Ref,
        _id: NodeId<tree::Ref>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_raw_inline(
        &mut self,
        _raw_inline: &tree::RawInline,
        _id: NodeId<tree::RawInline>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_math_inline(
        &mut self,
        _math_inline: &tree::MathInline,
        _id: NodeId<tree::MathInline>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_comment(
        &mut self,
        _comment: &tree::Comment,
        _id: NodeId<tree::Comment>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_escape(
        &mut self,
        _escape: &tree::Escape,
        _id: NodeId<tree::Escape>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_word(
        &mut self,
        _word: &tree::Word,
        _id: NodeId<tree::Word>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_spacing(
        &mut self,
        _spacing: &tree::Spacing,
        _id: NodeId<tree::Spacing>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_soft_break(
        &mut self,
        _soft_break: &tree::SoftBreak,
        _id: NodeId<tree::SoftBreak>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_code(
        &mut self,
        _code: &tree::Code,
        _id: NodeId<tree::Code>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_expr(
        &mut self,
        _expr: &tree::Expr,
        _id: NodeId<tree::Expr>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_expr_block(
        &mut self,
        _expr_block: &tree::ExprBlock,
        _id: NodeId<tree::ExprBlock>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_ident(
        &mut self,
        _ident: &tree::Ident,
        _id: NodeId<tree::Ident>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_call(
        &mut self,
        _call: &tree::Call,
        _id: NodeId<tree::Call>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_args(
        &mut self,
        _args: &tree::Args,
        _id: NodeId<tree::Args>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_arg(&mut self, _arg: &tree::Arg, _id: NodeId<tree::Arg>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_literal(
        &mut self,
        _literal: &tree::Literal,
        _id: NodeId<tree::Literal>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn handle_content(
        &mut self,
        _content: &tree::Content,
        _id: NodeId<tree::Content>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
