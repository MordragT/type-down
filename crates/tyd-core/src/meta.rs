use crate::{id::NodeId, tree::*};

use std::{fmt::Debug, marker::PhantomData, ops::Deref, sync::Arc};

pub trait Phase: 'static + Debug + Copy {
    type Error: Debug + Clone;
    type Tag: Debug + Clone;
    type Text: Debug + Clone;
    type Label: Debug + Clone;

    // Block
    type Block: Debug + Clone;
    type Raw: Debug + Clone;
    type Heading: Debug + Clone;
    type HeadingMarker: Debug + Clone;
    type Table: Debug + Clone;
    type TableRow: Debug + Clone;
    type List: Debug + Clone;
    type ListItem: Debug + Clone;
    type Enum: Debug + Clone;
    type EnumItem: Debug + Clone;
    type Terms: Debug + Clone;
    type TermItem: Debug + Clone;
    type Paragraph: Debug + Clone;
    type Plain: Debug + Clone;

    // Inline
    type Inline: Debug + Clone;
    type Quote: Debug + Clone;
    type Strikeout: Debug + Clone;
    type Emphasis: Debug + Clone;
    type Strong: Debug + Clone;
    type Subscript: Debug + Clone;
    type Supscript: Debug + Clone;
    type Link: Debug + Clone;
    type Ref: Debug + Clone;
    type RawInline: Debug + Clone;
    type MathInline: Debug + Clone;
    type Comment: Debug + Clone;
    type Escape: Debug + Clone;
    type Word: Debug + Clone;
    type Spacing: Debug + Clone;
    type SoftBreak: Debug + Clone;

    // Code
    type Code: Debug + Clone;
    type Expr: Debug + Clone;
    type Let: Debug + Clone;
    type Bind: Debug + Clone;
    type If: Debug + Clone;
    type For: Debug + Clone;
    type Call: Debug + Clone;
    type Args: Debug + Clone;
    type Arg: Debug + Clone;
    type Literal: Debug + Clone;
    type Ident: Debug + Clone;
    type Content: Debug + Clone;
}

pub trait MetaCast<P: Phase> {
    type Meta;

    fn upcast(this: Self::Meta) -> Meta<P>;
    fn try_downcast(meta: Meta<P>) -> Option<Self::Meta>;
    fn try_downcast_ref(meta: &Meta<P>) -> Option<&Self::Meta>;
    fn try_downcast_mut(meta: &mut Meta<P>) -> Option<&mut Self::Meta>;
}

macro_rules! impl_meta_cast {
    ($($node:ident),*) => {
        $(
            impl<P: Phase> MetaCast<P> for $node {
                type Meta = P::$node;

                fn upcast(this: Self::Meta) -> Meta<P> {
                    Meta::$node(this)
                }

                fn try_downcast(meta: Meta<P>) -> Option<Self::Meta> {
                    match meta {
                        Meta::$node(val) => Some(val),
                        _ => None,
                    }
                }

                fn try_downcast_ref(meta: &Meta<P>) -> Option<&Self::Meta> {
                    match meta {
                        Meta::$node(val) => Some(val),
                        _ => None,
                    }
                }

                fn try_downcast_mut(meta: &mut Meta<P>) -> Option<&mut Self::Meta> {
                    match meta {
                        Meta::$node(val) => Some(val),
                        _ => None,
                    }
                }
            }
        )*
    };
}

impl_meta_cast!(
    Error,
    Tag,
    Text,
    Label,
    // Block
    Block,
    Raw,
    Heading,
    HeadingMarker,
    Table,
    TableRow,
    List,
    ListItem,
    Enum,
    EnumItem,
    Terms,
    TermItem,
    Paragraph,
    Plain,
    // Inline
    Inline,
    Quote,
    Strikeout,
    Emphasis,
    Strong,
    Subscript,
    Supscript,
    Link,
    Ref,
    RawInline,
    MathInline,
    Comment,
    Escape,
    Word,
    Spacing,
    SoftBreak,
    // Code
    Code,
    Expr,
    Let,
    Bind,
    If,
    For,
    Call,
    Args,
    Arg,
    Literal,
    Ident,
    Content
);

#[derive(Clone, Debug)]
pub enum Meta<P: Phase> {
    Error(<Error as MetaCast<P>>::Meta),
    Tag(<Tag as MetaCast<P>>::Meta),
    Text(<Text as MetaCast<P>>::Meta),
    Label(<Label as MetaCast<P>>::Meta),

    // Block
    Block(<Block as MetaCast<P>>::Meta),
    Raw(<Raw as MetaCast<P>>::Meta),
    Heading(<Heading as MetaCast<P>>::Meta),
    HeadingMarker(<HeadingMarker as MetaCast<P>>::Meta),
    Table(<Table as MetaCast<P>>::Meta),
    TableRow(<TableRow as MetaCast<P>>::Meta),
    List(<List as MetaCast<P>>::Meta),
    ListItem(<ListItem as MetaCast<P>>::Meta),
    Enum(<Enum as MetaCast<P>>::Meta),
    EnumItem(<EnumItem as MetaCast<P>>::Meta),
    Terms(<Terms as MetaCast<P>>::Meta),
    TermItem(<TermItem as MetaCast<P>>::Meta),
    Paragraph(<Paragraph as MetaCast<P>>::Meta),
    Plain(<Plain as MetaCast<P>>::Meta),

    // Inline
    Inline(<Inline as MetaCast<P>>::Meta),
    Quote(<Quote as MetaCast<P>>::Meta),
    Strikeout(<Strikeout as MetaCast<P>>::Meta),
    Emphasis(<Emphasis as MetaCast<P>>::Meta),
    Strong(<Strong as MetaCast<P>>::Meta),
    Subscript(<Subscript as MetaCast<P>>::Meta),
    Supscript(<Supscript as MetaCast<P>>::Meta),
    Link(<Link as MetaCast<P>>::Meta),
    Ref(<Ref as MetaCast<P>>::Meta),
    RawInline(<RawInline as MetaCast<P>>::Meta),
    MathInline(<MathInline as MetaCast<P>>::Meta),
    Comment(<Comment as MetaCast<P>>::Meta),
    Escape(<Escape as MetaCast<P>>::Meta),
    Word(<Word as MetaCast<P>>::Meta),
    Spacing(<Spacing as MetaCast<P>>::Meta),
    SoftBreak(<SoftBreak as MetaCast<P>>::Meta),

    // Code
    Code(<Code as MetaCast<P>>::Meta),
    Expr(<Expr as MetaCast<P>>::Meta),
    Let(<Let as MetaCast<P>>::Meta),
    Bind(<Bind as MetaCast<P>>::Meta),
    If(<If as MetaCast<P>>::Meta),
    For(<For as MetaCast<P>>::Meta),
    Call(<Call as MetaCast<P>>::Meta),
    Args(<Args as MetaCast<P>>::Meta),
    Arg(<Arg as MetaCast<P>>::Meta),
    Literal(<Literal as MetaCast<P>>::Meta),
    Ident(<Ident as MetaCast<P>>::Meta),
    Content(<Content as MetaCast<P>>::Meta),
}

impl<P, M> Meta<P>
where
    M: Clone + Debug,
    P: Phase<
            Error = M,
            Tag = M,
            Text = M,
            Label = M,
            // Block
            Block = M,
            Raw = M,
            Heading = M,
            HeadingMarker = M,
            Table = M,
            TableRow = M,
            List = M,
            ListItem = M,
            Enum = M,
            EnumItem = M,
            Terms = M,
            TermItem = M,
            Paragraph = M,
            Plain = M,
            // Inline
            Inline = M,
            Quote = M,
            Strikeout = M,
            Emphasis = M,
            Strong = M,
            Subscript = M,
            Supscript = M,
            Link = M,
            Ref = M,
            RawInline = M,
            MathInline = M,
            Comment = M,
            Escape = M,
            Word = M,
            Spacing = M,
            SoftBreak = M,
            // Code
            Code = M,
            Expr = M,
            Let = M,
            Bind = M,
            If = M,
            For = M,
            Call = M,
            Args = M,
            Arg = M,
            Literal = M,
            Ident = M,
            Content = M,
        >,
{
    pub fn inner_copied(&self) -> M
    where
        M: Copy,
    {
        match self {
            Self::Error(m) => *m,
            Self::Tag(m) => *m,
            Self::Text(m) => *m,
            Self::Label(m) => *m,

            // Block
            Self::Block(m) => *m,
            Self::Raw(m) => *m,
            Self::Heading(m) => *m,
            Self::HeadingMarker(m) => *m,
            Self::Table(m) => *m,
            Self::TableRow(m) => *m,
            Self::List(m) => *m,
            Self::ListItem(m) => *m,
            Self::Enum(m) => *m,
            Self::EnumItem(m) => *m,
            Self::Terms(m) => *m,
            Self::TermItem(m) => *m,
            Self::Paragraph(m) => *m,
            Self::Plain(m) => *m,

            // Inline
            Self::Inline(m) => *m,
            Self::Quote(m) => *m,
            Self::Strikeout(m) => *m,
            Self::Emphasis(m) => *m,
            Self::Strong(m) => *m,
            Self::Subscript(m) => *m,
            Self::Supscript(m) => *m,
            Self::Link(m) => *m,
            Self::Ref(m) => *m,
            Self::RawInline(m) => *m,
            Self::MathInline(m) => *m,
            Self::Comment(m) => *m,
            Self::Escape(m) => *m,
            Self::Word(m) => *m,
            Self::Spacing(m) => *m,
            Self::SoftBreak(m) => *m,

            // Code
            Self::Code(m) => *m,
            Self::Expr(m) => *m,
            Self::Let(m) => *m,
            Self::Bind(m) => *m,
            Self::If(m) => *m,
            Self::For(m) => *m,
            Self::Call(m) => *m,
            Self::Args(m) => *m,
            Self::Arg(m) => *m,
            Self::Literal(m) => *m,
            Self::Ident(m) => *m,
            Self::Content(m) => *m,
        }
    }

    pub fn inner_ref(&self) -> &M {
        match self {
            Self::Error(m) => m,
            Self::Tag(m) => m,
            Self::Text(m) => m,
            Self::Label(m) => m,

            // Block
            Self::Block(m) => m,
            Self::Raw(m) => m,
            Self::Heading(m) => m,
            Self::HeadingMarker(m) => m,
            Self::Table(m) => m,
            Self::TableRow(m) => m,
            Self::List(m) => m,
            Self::ListItem(m) => m,
            Self::Enum(m) => m,
            Self::EnumItem(m) => m,
            Self::Terms(m) => m,
            Self::TermItem(m) => m,
            Self::Paragraph(m) => m,
            Self::Plain(m) => m,

            // Inline
            Self::Inline(m) => m,
            Self::Quote(m) => m,
            Self::Strikeout(m) => m,
            Self::Emphasis(m) => m,
            Self::Strong(m) => m,
            Self::Subscript(m) => m,
            Self::Supscript(m) => m,
            Self::Link(m) => m,
            Self::Ref(m) => m,
            Self::RawInline(m) => m,
            Self::MathInline(m) => m,
            Self::Comment(m) => m,
            Self::Escape(m) => m,
            Self::Word(m) => m,
            Self::Spacing(m) => m,
            Self::SoftBreak(m) => m,

            // Code
            Self::Code(m) => m,
            Self::Expr(m) => m,
            Self::Let(m) => m,
            Self::Bind(m) => m,
            Self::If(m) => m,
            Self::For(m) => m,
            Self::Call(m) => m,
            Self::Args(m) => m,
            Self::Arg(m) => m,
            Self::Literal(m) => m,
            Self::Ident(m) => m,
            Self::Content(m) => m,
        }
    }

    pub fn inner_mut(&mut self) -> &mut M {
        match self {
            Self::Error(m) => m,
            Self::Tag(m) => m,
            Self::Text(m) => m,
            Self::Label(m) => m,

            // Block
            Self::Block(m) => m,
            Self::Raw(m) => m,
            Self::Heading(m) => m,
            Self::HeadingMarker(m) => m,
            Self::Table(m) => m,
            Self::TableRow(m) => m,
            Self::List(m) => m,
            Self::ListItem(m) => m,
            Self::Enum(m) => m,
            Self::EnumItem(m) => m,
            Self::Terms(m) => m,
            Self::TermItem(m) => m,
            Self::Paragraph(m) => m,
            Self::Plain(m) => m,

            // Inline
            Self::Inline(m) => m,
            Self::Quote(m) => m,
            Self::Strikeout(m) => m,
            Self::Emphasis(m) => m,
            Self::Strong(m) => m,
            Self::Subscript(m) => m,
            Self::Supscript(m) => m,
            Self::Link(m) => m,
            Self::Ref(m) => m,
            Self::RawInline(m) => m,
            Self::MathInline(m) => m,
            Self::Comment(m) => m,
            Self::Escape(m) => m,
            Self::Word(m) => m,
            Self::Spacing(m) => m,
            Self::SoftBreak(m) => m,

            // Code
            Self::Code(m) => m,
            Self::Expr(m) => m,
            Self::Let(m) => m,
            Self::Bind(m) => m,
            Self::If(m) => m,
            Self::For(m) => m,
            Self::Call(m) => m,
            Self::Args(m) => m,
            Self::Arg(m) => m,
            Self::Literal(m) => m,
            Self::Ident(m) => m,
            Self::Content(m) => m,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Metadata<P, C = MetaVec<P>>
where
    P: Phase,
    C: MetaContainer<P>,
{
    container: Arc<C>,
    phase: PhantomData<P>,
}

impl<P, C> From<C> for Metadata<P, C>
where
    P: Phase,
    C: MetaContainer<P>,
{
    fn from(value: C) -> Self {
        Self {
            container: Arc::new(value),
            phase: PhantomData,
        }
    }
}

impl<P, C> Deref for Metadata<P, C>
where
    P: Phase,
    C: MetaContainer<P>,
{
    type Target = Arc<C>;

    fn deref(&self) -> &Self::Target {
        &self.container
    }
}

pub type MetaVec<P> = Vec<Meta<P>>;

pub trait MetaContainer<P: Phase>: Sized {
    fn get<T>(&self, id: NodeId<T>) -> &Meta<P>;
    fn get_mut<T>(&mut self, id: NodeId<T>) -> &mut Meta<P>;

    fn insert_meta<T>(&mut self, id: NodeId<T>, meta: T::Meta)
    where
        T: MetaCast<P>;

    fn meta<T>(&self, id: NodeId<T>) -> &T::Meta
    where
        T: MetaCast<P>,
    {
        let meta = self.get(id);
        T::try_downcast_ref(meta).unwrap()
    }

    fn meta_mut<T>(&mut self, id: NodeId<T>) -> &mut T::Meta
    where
        T: MetaCast<P>,
    {
        let meta = self.get_mut(id);
        T::try_downcast_mut(meta).unwrap()
    }

    fn update_meta<T>(&mut self, id: NodeId<T>, meta: T::Meta) -> T::Meta
    where
        T: MetaCast<P>,
    {
        std::mem::replace(self.meta_mut(id), meta)
    }
}

impl<P: Phase> MetaContainer<P> for MetaVec<P> {
    fn get<T>(&self, id: NodeId<T>) -> &Meta<P> {
        &self[id.as_usize()]
    }

    fn get_mut<T>(&mut self, id: NodeId<T>) -> &mut Meta<P> {
        &mut self[id.as_usize()]
    }

    fn insert_meta<T>(&mut self, id: NodeId<T>, meta: T::Meta)
    where
        T: MetaCast<P>,
    {
        assert_eq!(self.len(), id.as_usize());

        self.push(T::upcast(meta));
    }
}

#[derive(Debug, Clone)]
pub struct MetaCache<P: Phase>(Vec<Option<Meta<P>>>);

impl<P: Phase> MetaCache<P> {
    pub fn new(size: usize) -> Self {
        Self(vec![None; size])
    }

    pub fn take<T>(&mut self, id: NodeId<T>) -> T::Meta
    where
        T: MetaCast<P>,
    {
        let meta = self.0[id.as_usize()].take().unwrap();

        T::try_downcast(meta).unwrap()
    }
}

impl<P: Phase> MetaContainer<P> for MetaCache<P> {
    fn get<T>(&self, id: NodeId<T>) -> &Meta<P> {
        self.0[id.as_usize()].as_ref().unwrap()
    }

    fn get_mut<T>(&mut self, id: NodeId<T>) -> &mut Meta<P> {
        self.0[id.as_usize()].as_mut().unwrap()
    }

    fn insert_meta<T>(&mut self, id: NodeId<T>, meta: T::Meta)
    where
        T: MetaCast<P>,
    {
        self.0[id.as_usize()].replace(T::upcast(meta));
    }
}
