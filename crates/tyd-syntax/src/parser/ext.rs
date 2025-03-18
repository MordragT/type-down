use chumsky::{combinator::*, prelude::*};
use ecow::EcoString;
use tyd_core::prelude::*;

use crate::{Span, SyntaxPhase};

use super::extra::{Extra, State};

pub trait ParserExt<'src, T>: Parser<'src, &'src str, T, Extra<'src>> + Sized {
    #[inline]
    fn to_ecow(self) -> Map<ToSlice<Self, T>, &'src str, impl Fn(&'src str) -> EcoString> {
        self.to_slice().map(EcoString::from)
    }

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

    #[inline]
    fn map_to_node<F, U>(self, f: F) -> impl Parser<'src, &'src str, NodeId<U>, Extra<'src>>
    where
        F: Fn(T) -> U,
        U: MetaCast<SyntaxPhase, Meta = Span>,
        Node: From<U>,
    {
        self.map(f).to_node()
    }

    #[inline]
    fn to_inline(self) -> impl Parser<'src, &'src str, NodeId<tree::Inline>, Extra<'src>>
    where
        tree::Inline: From<T>,
    {
        self.map(tree::Inline::from).to_node()
    }

    #[inline]
    fn to_block(self) -> impl Parser<'src, &'src str, NodeId<tree::Block>, Extra<'src>>
    where
        tree::Block: From<T>,
    {
        self.map(tree::Block::from).to_node()
    }

    #[inline]
    fn to_expr(self) -> impl Parser<'src, &'src str, NodeId<tree::Expr>, Extra<'src>>
    where
        tree::Expr: From<T>,
    {
        self.map(tree::Expr::from).to_node()
    }
}

impl<'src, T, P: Parser<'src, &'src str, T, Extra<'src>> + Sized> ParserExt<'src, T> for P {}
