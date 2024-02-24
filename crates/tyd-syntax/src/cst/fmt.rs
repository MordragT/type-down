use std::fmt::{self, Write};

use super::*;

impl fmt::Display for Cst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (block, _) in &self.0 .0 {
            write!(f, "{block}\n")?;
        }

        Ok(())
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Raw(raw) => raw.fmt(f),
            Self::Heading(heading) => heading.fmt(f),
            Self::BulletList(bullet) => bullet.fmt(f),
            Self::OrderedList(ordered) => ordered.fmt(f),
            Self::Table(table) => table.fmt(f),
            Self::BlockQuote(block_quote) => block_quote.fmt(f),
            Self::Paragraph(paragraph) => paragraph.fmt(f),
        }
    }
}

impl fmt::Display for Raw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(l, lang, _, content, r, _) = &self;

        let lang = match lang {
            Some(lang) => format!(" {}", lang.0 .0),
            None => String::new(),
        };

        write!(f, "{l}{lang}\n{}{r}\n", content.0)
    }
}

impl fmt::Display for Heading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
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

impl fmt::Display for BulletList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (minus, line) in &self.0 .0 {
            write!(f, "{minus} {line}")?;
        }

        Ok(())
    }
}

impl fmt::Display for OrderedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (plus, line) in &self.0 .0 {
            write!(f, "{plus} {line}")?;
        }

        Ok(())
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 .0 {
            row.fmt(f)?;
        }

        Ok(())
    }
}

impl fmt::Display for TableRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)?;

        for (elements, pipe) in &self.1 .0 {
            write!(f, " {elements} {pipe}")?;
        }

        write!(f, "\n")
    }
}

impl fmt::Display for BlockQuote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (right_angle, line) in &self.0 .0 {
            write!(f, "{right_angle} {line}")?;
        }

        Ok(())
    }
}

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.0 .0 {
            line.fmt(f)?;
        }

        Ok(())
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(elements, label, _) = &self;

        let label = match label {
            Some(label) => format!(" {label}"),
            None => String::new(),
        };

        write!(f, "{elements}{label}\n")
    }
}

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
            Self::Access(access) => access.fmt(f),
            Self::Quote(quote) => quote.fmt(f),
            Self::Strikeout(strikeout) => strikeout.fmt(f),
            Self::Emphasis(emphasis) => emphasis.fmt(f),
            Self::Strong(strong) => strong.fmt(f),
            Self::Enclosed(enclosed) => enclosed.fmt(f),
            Self::Link(link) => link.fmt(f),
            Self::Escape(escape) => escape.fmt(f),
            Self::RawInline(raw_inline) => raw_inline.fmt(f),
            Self::Spacing(spacing) => spacing.fmt(f),
            Self::SubScript(script) => script.fmt(f),
            Self::SupScript(script) => script.fmt(f),
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
            Self::Access(access) => access.fmt(f),
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
            Self::Access(access) => access.fmt(f),
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
            Self::Access(access) => access.fmt(f),
            Self::Spacing(spacing) => spacing.fmt(f),
            Self::SubScript(script) => script.fmt(f),
            Self::SupScript(script) => script.fmt(f),
            Self::Word(word) => word.fmt(f),
        }
    }
}

impl fmt::Display for Enclosed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(l, elements, r) = &self;

        write!(f, "{l}{}{r}", elements.0)
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

impl fmt::Display for Access {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(pound, ident, call) = &self;

        pound.fmt(f)?;
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
        let Self(ident, colon, value) = &self;
        write!(f, "{}{colon}{value}", ident.0)
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
