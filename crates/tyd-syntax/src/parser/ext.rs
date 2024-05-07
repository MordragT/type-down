use chumsky::{combinator::*, input::MapExtra, prelude::*};
use ecow::EcoString;

use super::Extra;
use crate::{
    kind::SyntaxKind,
    node::{BranchNode, ErrorNode, LeafNode, Node},
};

pub trait LeafParser<'src, T>: Parser<'src, &'src str, T, Extra<'src>> + Sized {
    #[inline]
    fn to_ecow(self) -> Map<ToSlice<Self, T>, &'src str, impl Fn(&'src str) -> EcoString> {
        self.to_slice().map(EcoString::from)
    }

    #[inline]
    fn to_error(
        self,
    ) -> MapWith<
        Map<ToSlice<Self, T>, &'src str, impl Fn(&'src str) -> EcoString>,
        EcoString,
        impl for<'a, 'b> Fn(EcoString, &'a mut MapExtra<'src, 'b, &'src str, Extra<'src>>) -> Node,
    > {
        self.to_ecow().map_with(|text, e| {
            Node::Error(ErrorNode {
                span: e.span(),
                text,
            })
        })
    }

    #[inline]
    fn to_leaf_node(
        self,
        kind: SyntaxKind,
    ) -> MapWith<
        Map<ToSlice<Self, T>, &'src str, impl Fn(&'src str) -> EcoString>,
        EcoString,
        impl for<'a, 'b> Fn(EcoString, &'a mut MapExtra<'src, 'b, &'src str, Extra<'src>>) -> LeafNode,
    > {
        self.to_ecow().map_with(move |content, e| LeafNode {
            span: e.span(),
            kind,
            text: content,
        })
    }

    #[inline]
    fn to_leaf(
        self,
        kind: SyntaxKind,
    ) -> MapWith<
        Map<ToSlice<Self, T>, &'src str, impl Fn(&'src str) -> EcoString>,
        EcoString,
        impl for<'a, 'b> Fn(EcoString, &'a mut MapExtra<'src, 'b, &'src str, Extra<'src>>) -> Node,
    > {
        self.to_ecow().map_with(move |content, e| {
            Node::Leaf(LeafNode {
                span: e.span(),
                kind,
                text: content,
            })
        })
    }
}

impl<'src, T, P: Parser<'src, &'src str, T, Extra<'src>> + Sized> LeafParser<'src, T> for P {}

pub trait BranchParser<'src>: Parser<'src, &'src str, Vec<Node>, Extra<'src>> + Sized {
    #[inline]
    fn to_branch_node(
        self,
        kind: SyntaxKind,
    ) -> MapWith<
        Self,
        Vec<Node>,
        impl for<'a, 'b> Fn(Vec<Node>, &'a mut MapExtra<'src, 'b, &'src str, Extra<'src>>) -> BranchNode,
    > {
        self.map_with(move |content, e| BranchNode {
            span: e.span(),
            kind,
            children: content,
        })
    }

    #[inline]
    fn to_branch(
        self,
        kind: SyntaxKind,
    ) -> MapWith<
        Self,
        Vec<Node>,
        impl for<'a, 'b> Fn(Vec<Node>, &'a mut MapExtra<'src, 'b, &'src str, Extra<'src>>) -> Node,
    > {
        self.map_with(move |content, e| {
            Node::Branch(BranchNode {
                span: e.span(),
                kind,
                children: content,
            })
        })
    }
}

impl<'src, P: Parser<'src, &'src str, Vec<Node>, Extra<'src>> + Sized> BranchParser<'src> for P {}
