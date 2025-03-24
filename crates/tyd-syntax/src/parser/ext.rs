use chumsky::{combinator::*, prelude::*};
use ecow::EcoString;
use tyd_core::prelude::*;

use crate::{Span, SyntaxPhase};

use super::extra::{Extra, State};

/// Extension methods for the Chumsky parser
///
/// This trait provides useful extension methods for parsers in the context
/// of the application, allowing conversion to various data structures and nodes.
pub trait ParserExt<'src, T>: Parser<'src, &'src str, T, Extra<'src>> + Sized {
    /// Converts the parser's output to an `EcoString`
    ///
    /// This is useful for efficiently handling string data in the parser.
    #[inline]
    fn to_ecow(self) -> Map<ToSlice<Self, T>, &'src str, impl Fn(&'src str) -> EcoString> {
        self.to_slice().map(EcoString::from)
    }

    /// Converts the parser's output to a node in the syntax tree
    ///
    /// This creates a new node with the current span information and inserts it
    /// into the parse state.
    #[inline]
    fn to_node(self) -> impl Parser<'src, &'src str, NodeId<T>, Extra<'src>>
    where
        T: MetaCast<SyntaxPhase, Meta = Span>,
        Node: From<T>,
    {
        self.map_with(|node, e| {
            let span = e.span();
            let state: &mut State = e.state();

            state.insert(node, span)
        })
    }

    /// Maps the parser's output to a different type and then converts it to a node
    ///
    /// This is a convenience method that combines `map` and `to_node`.
    #[inline]
    fn map_to_node<F, U>(self, f: F) -> impl Parser<'src, &'src str, NodeId<U>, Extra<'src>>
    where
        F: Fn(T) -> U,
        U: MetaCast<SyntaxPhase, Meta = Span>,
        Node: From<U>,
    {
        self.map(f).to_node()
    }

    /// Converts the parser's output to an inline element in the syntax tree
    ///
    /// This is a convenience method for creating inline elements.
    #[inline]
    fn to_inline(self) -> impl Parser<'src, &'src str, NodeId<tree::Inline>, Extra<'src>>
    where
        tree::Inline: From<T>,
    {
        self.map(tree::Inline::from).to_node()
    }

    /// Converts the parser's output to a block element in the syntax tree
    ///
    /// This is a convenience method for creating block elements.
    #[inline]
    fn to_block(self) -> impl Parser<'src, &'src str, NodeId<tree::Block>, Extra<'src>>
    where
        tree::Block: From<T>,
    {
        self.map(tree::Block::from).to_node()
    }

    /// Converts the parser's output to an expression in the syntax tree
    ///
    /// This is a convenience method for creating expression nodes.
    #[inline]
    fn to_expr(self) -> impl Parser<'src, &'src str, NodeId<tree::Expr>, Extra<'src>>
    where
        tree::Expr: From<T>,
    {
        self.map(tree::Expr::from).to_node()
    }
}

/// Implements the `ParserExt` trait for all types that implement the `Parser` trait
///
/// This blanket implementation ensures that all parsers can use the extension methods.
impl<'src, T, P: Parser<'src, &'src str, T, Extra<'src>> + Sized> ParserExt<'src, T> for P {}
