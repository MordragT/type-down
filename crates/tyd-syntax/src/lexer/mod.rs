use chumsky::{error::Rich, extra, Parser};
use miette::NamedSource;
use std::fmt;

use crate::{ast::*, Span};

use self::{
    error::LexErrors,
    markup::{nodes_parser, nodes_spanned_parser},
};

pub mod code;
pub mod error;
pub mod markup;

type Extra<'src> = extra::Err<Rich<'src, char, Span>>;

pub fn lex<'src>(src: &'src str, name: impl AsRef<str>) -> Result<Vec<Node<'src>>, LexErrors> {
    let parser = nodes_parser();

    let nodes = parser.parse(&src).into_result().map_err(|errs| LexErrors {
        src: NamedSource::new(name, src.to_owned()),
        related: errs.into_iter().map(Into::into).collect(),
    })?;

    Ok(nodes)
}

pub fn lex_spanned<'src>(
    src: &'src str,
    name: impl AsRef<str>,
) -> Result<Vec<(Node<'src>, Span)>, LexErrors> {
    let parser = nodes_spanned_parser();

    let nodes = parser.parse(&src).into_result().map_err(|errs| LexErrors {
        src: NamedSource::new(name, src.to_owned()),
        related: errs.into_iter().map(Into::into).collect(),
    })?;

    Ok(nodes)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<'src> {
    Heading(Heading<'src>),
    Div(Div<'src>),
    Raw(Raw<'src>),
    TableRow(TableRow<'src>),
    ListItem(ListItem<'src>),
    EnumItem(EnumItem<'src>),
    TermItem(TermItem<'src>),
    Text(Text<'src>),
    // Label(&'src str),
    LineBreak,
    Indentation,
}

impl<'src> fmt::Display for Node<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Heading(_) => f.write_str("Heading"),
            Self::Div(_) => f.write_str("Div"),
            Self::Raw(_) => f.write_str("Raw"),
            Self::TableRow(_) => f.write_str("TableRow"),
            Self::ListItem(_) => f.write_str("ListItem"),
            Self::EnumItem(_) => f.write_str("EnumItem"),
            Self::TermItem(_) => f.write_str("BlockQuoteItem"),
            Self::Text(_) => f.write_str("Text"),
            Self::LineBreak => f.write_str("LineBreak"),
            Self::Indentation => f.write_str("Indentation"),
        }
    }
}
