use std::fmt::{self, Write};

use super::*;

impl fmt::Display for Cst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Nodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for node in &self.0 .0 {
            buffer.push_str(&node.to_string());
        }

        buffer.fmt(f)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Raw(raw) => raw.fmt(f),
            Self::Heading(heading) => heading.fmt(f),
            Self::BlockQuote(block_quote) => block_quote.fmt(f),
            Self::ListItem(item) => item.fmt(f),
            Self::TableRow(row) => row.fmt(f),
            Self::Label(label) => label.fmt(f),
            Self::LineBreak(_) => "\n".fmt(f),
            Self::Plain(elements) => elements.fmt(f),
        }
    }
}

impl fmt::Display for Raw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let Self(l, lang, _, content, r, _) = &self;

        // let lang = match lang {
        //     Some(lang) => format!(" {}", lang.0 .0),
        //     None => String::new(),
        // };

        // write!(f, "{l}{lang}\n{}{r}\n", content.0)

        let Self(l, content, r) = &self;

        write!(f, "{l}{}{r}", content.0)
    }
}

impl fmt::Display for Heading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(delim, space, elements) = &self;
        write!(f, "{delim}{space}{elements}")
    }
}

impl fmt::Display for HeadingLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for _ in 0..self.0 .0.len() {
            buffer.push('=');
        }

        buffer.fmt(f)
    }
}

impl fmt::Display for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(indent, delim, space, content) = &self;

        if let Some(indent) = indent {
            indent.fmt(f)?;
        }

        write!(f, "{delim}{space}{content}")
    }
}

impl fmt::Display for ListItemDelim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minus(minus) => minus.fmt(f),
            Self::Plus(plus) => plus.fmt(f),
        }
    }
}

impl fmt::Display for ListItemContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlockQuote(block_quote) => block_quote.0.fmt(f),
            Self::Plain(elements) => elements.fmt(f),
        }
    }
}

// impl fmt::Display for BulletList {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for (minus, line) in &self.0 .0 {
//             write!(f, "{minus} {line}")?;
//         }

//         Ok(())
//     }
// }

// impl fmt::Display for OrderedList {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for (plus, line) in &self.0 .0 {
//             write!(f, "{plus} {line}")?;
//         }

//         Ok(())
//     }
// }

// impl fmt::Display for Table {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for row in &self.0 .0 {
//             row.fmt(f)?;
//         }

//         Ok(())
//     }
// }

impl fmt::Display for TableRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;

        for (cell, pipe) in &self.1 .0 {
            write!(f, "{cell}{pipe}")?;
        }

        write!(f, "\n")
    }
}

impl fmt::Display for TableCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlockQuote(block_quote) => block_quote.fmt(f),
            Self::ListItem(item) => item.fmt(f),
            Self::Plain(elements) => elements.fmt(f),
        }
    }
}

impl fmt::Display for BlockQuote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(level, space, item) = &self;

        write!(f, "{level}{space}{item}")
    }
}

impl fmt::Display for BlockQuoteLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for delim in &self.0 .0 {
            buffer.push('>');
        }

        buffer.fmt(f)
    }
}

impl fmt::Display for BlockQuoteItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ListItem(item) => item.fmt(f),
            Self::Plain(elements) => elements.fmt(f),
        }
    }
}

// impl fmt::Display for Paragraph {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for line in &self.0 .0 {
//             line.fmt(f)?;
//         }

//         Ok(())
//     }
// }

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(delim, content) = &self;
        write!(f, "{delim}{}", content.0)
    }
}

// impl fmt::Display for Line {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let Self(elements, label, _) = &self;

//         let label = match label {
//             Some(label) => format!(" {label}"),
//             None => String::new(),
//         };

//         write!(f, "{elements}{label}\n")
//     }
// }

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(at, ident) = &self;

        write!(f, "{at}{}", ident.0)
    }
}

impl fmt::Display for Elements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for el in &self.0 .0 {
            buffer.push_str(&el.to_string());
        }

        buffer.trim().fmt(f)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Code(code) => code.fmt(f),
            Self::Quote(quote) => quote.fmt(f),
            Self::Strikeout(strikeout) => strikeout.fmt(f),
            Self::Emphasis(emphasis) => emphasis.fmt(f),
            Self::Strong(strong) => strong.fmt(f),
            Self::Link(link) => link.fmt(f),
            Self::Escape(escape) => escape.fmt(f),
            Self::RawInline(raw_inline) => raw_inline.fmt(f),
            Self::Spacing(spacing) => spacing.fmt(f),
            Self::SubScript(script) => script.fmt(f),
            Self::SupScript(script) => script.fmt(f),
            Self::Comment(comment) => comment.fmt(f),
            Self::Word(word) => word.fmt(f),
        }
    }
}

impl fmt::Display for SubScript {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        self.1.fmt(f)
    }
}

impl fmt::Display for SupScript {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;
        self.1.fmt(f)
    }
}

impl fmt::Display for Spacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for space in &self.0 .0 {
            space.fmt(f)?;
        }

        Ok(())
    }
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(l, content, r) = &self;

        let mut output = String::new();

        for el in &content.0 {
            write!(output, "{el}")?;
        }

        write!(f, "{l}{}{r}", output.trim())
    }
}

impl fmt::Display for QuoteElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Code(code) => code.fmt(f),
            Self::Escape(escape) => escape.fmt(f),
            Self::Strikeout(strikeout) => strikeout.fmt(f),
            Self::Emphasis(emphasis) => emphasis.fmt(f),
            Self::Strong(strong) => strong.fmt(f),
            Self::Spacing(spacing) => spacing.fmt(f),
            Self::SubScript(script) => script.fmt(f),
            Self::SupScript(script) => script.fmt(f),
            Self::Word(word) => word.fmt(f),
        }
    }
}

impl fmt::Display for Strikeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(l, content, r) = &self;

        let mut output = String::new();

        for el in &content.0 {
            write!(output, "{el}")?;
        }

        write!(f, "{l}{}{r}", output.trim())
    }
}

impl fmt::Display for StrikeoutElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Code(code) => code.fmt(f),
            Self::Escape(escape) => escape.fmt(f),
            Self::Emphasis(emphasis) => emphasis.fmt(f),
            Self::Strong(strong) => strong.fmt(f),
            Self::Spacing(spacing) => spacing.fmt(f),
            Self::SubScript(script) => script.fmt(f),
            Self::SupScript(script) => script.fmt(f),
            Self::Word(word) => word.fmt(f),
        }
    }
}

impl fmt::Display for Emphasis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(l, content, r) = &self;

        let mut output = String::new();

        for el in &content.0 {
            write!(output, "{el}")?;
        }

        write!(f, "{l}{}{r}", output.trim())
    }
}

impl fmt::Display for Strong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(l, content, r) = &self;

        let mut output = String::new();

        for el in &content.0 {
            write!(output, "{el}")?;
        }

        write!(f, "{l}{}{r}", output.trim())
    }
}

impl fmt::Display for EmphasizedElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Code(code) => code.fmt(f),
            Self::Escape(escape) => escape.fmt(f),
            Self::Spacing(spacing) => spacing.fmt(f),
            Self::SubScript(script) => script.fmt(f),
            Self::SupScript(script) => script.fmt(f),
            Self::Word(word) => word.fmt(f),
        }
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(l, nodes, r) = &self;

        write!(f, "{l}{}{r}", nodes.0)
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(l, content, r, enclosed) = &self;

        write!(f, "{l}{content}{r}")?;

        if let Some(enclosed) = enclosed {
            enclosed.fmt(f)?;
        }

        Ok(())
    }
}

impl fmt::Display for Escape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(back_slash, any) = &self;

        write!(f, "{back_slash}{}", any.0)
    }
}

impl fmt::Display for RawInline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(l, content, r) = &self;

        write!(f, "{l}{content}{r}")
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(pound, expr) = &self;

        write!(f, "{pound}{expr}")
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Access(access) => access.fmt(f),
            Self::Content(content) => content.fmt(f),
        }
    }
}

impl fmt::Display for Access {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(ident, call) = &self;

        ident.0.fmt(f)?;

        if let Some(call) = call {
            call.fmt(f)?;
        }

        Ok(())
    }
}

impl fmt::Display for CallTail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(l, args, r, enclosed) = &self;

        write!(f, "{l}{args}{r}")?;

        if let Some(enclosed) = enclosed {
            enclosed.fmt(f)?;
        }

        Ok(())
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for arg in &self.0 .0 {
            buffer.push_str(&arg.to_string());
            buffer.push(',');
        }

        buffer.pop();

        buffer.fmt(f)
    }
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(named, _, value) = &self;

        if let Some((ident, colon)) = named {
            write!(f, "{}{colon}", ident.0)?;
        }

        value.fmt(f)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Identifier(ident) => ident.0.fmt(f),
            Self::String(s) => s.fmt(f),
        }
    }
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(l, content, r) = &self;
        write!(f, "{l}{content}{r}")
    }
}
