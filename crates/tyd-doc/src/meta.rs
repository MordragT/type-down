use crate::{id::NodeId, tree::*};

use std::{fmt::Debug, sync::Arc};

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
    type ExprBlock: Debug + Clone;
    type Ident: Debug + Clone;
    type Call: Debug + Clone;
    type Args: Debug + Clone;
    type Arg: Debug + Clone;
    type Literal: Debug + Clone;
    type Content: Debug + Clone;
}

pub trait MetaCast<P: Phase> {
    type Meta;

    fn upcast(this: Self::Meta) -> Meta<P>;
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
    ExprBlock,
    Ident,
    Call,
    Args,
    Arg,
    Literal,
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
    ExprBlock(<ExprBlock as MetaCast<P>>::Meta),
    Ident(<Ident as MetaCast<P>>::Meta),
    Call(<Call as MetaCast<P>>::Meta),
    Args(<Args as MetaCast<P>>::Meta),
    Arg(<Arg as MetaCast<P>>::Meta),
    Literal(<Literal as MetaCast<P>>::Meta),
    Content(<Content as MetaCast<P>>::Meta),
}

#[derive(Debug, Clone)]
pub struct Metadata<P: Phase>(Arc<Vec<Meta<P>>>);

impl<P: Phase> Metadata<P> {
    #[inline]
    pub fn new(data: Vec<Meta<P>>) -> Self {
        Self(Arc::new(data))
    }

    #[inline]
    pub fn get(&self, id: usize) -> &Meta<P> {
        &self.0[id]
    }

    #[inline]
    pub fn meta<T>(&self, id: NodeId<T>) -> &T::Meta
    where
        T: MetaCast<P>,
    {
        let meta = self.get(id.as_usize());
        T::try_downcast_ref(meta).unwrap()
    }
}

impl<P: Phase> From<Vec<Meta<P>>> for Metadata<P> {
    fn from(value: Vec<Meta<P>>) -> Self {
        Self(Arc::new(value))
    }
}
