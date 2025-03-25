use crate::{id::NodeId, tree::*};

use std::{fmt::Debug, marker::PhantomData, ops::Deref, sync::Arc};

/// A trait for phases that use a single metadata type for all node kinds.
///
/// This simplifies the implementation of `Phase` by allowing a single metadata type
/// to be used for all node kinds in the document tree.
pub trait UniformPhase: 'static + Debug + Copy {
    /// The metadata type to use for all nodes in this phase
    type Meta: Debug + Clone;
}

impl<T: UniformPhase> Phase for T {
    type Error = T::Meta;
    type Tag = T::Meta;
    type Text = T::Meta;
    type Label = T::Meta;

    // Block
    type Block = T::Meta;
    type Raw = T::Meta;
    type Heading = T::Meta;
    type HeadingMarker = T::Meta;
    type Table = T::Meta;
    type TableRow = T::Meta;
    type List = T::Meta;
    type ListItem = T::Meta;
    type Enum = T::Meta;
    type EnumItem = T::Meta;
    type Terms = T::Meta;
    type TermItem = T::Meta;
    type Paragraph = T::Meta;
    type Plain = T::Meta;

    // Inline
    type Inline = T::Meta;
    type Quote = T::Meta;
    type Strikeout = T::Meta;
    type Emphasis = T::Meta;
    type Strong = T::Meta;
    type Subscript = T::Meta;
    type Supscript = T::Meta;
    type Link = T::Meta;
    type Ref = T::Meta;
    type RawInline = T::Meta;
    type MathInline = T::Meta;
    type Comment = T::Meta;
    type Escape = T::Meta;
    type Word = T::Meta;
    type Spacing = T::Meta;
    type SoftBreak = T::Meta;

    // Code
    type Code = T::Meta;
    type Expr = T::Meta;
    type Let = T::Meta;
    type Bind = T::Meta;
    type If = T::Meta;
    type For = T::Meta;
    type Call = T::Meta;
    type Args = T::Meta;
    type Arg = T::Meta;
    type Literal = T::Meta;
    type Ident = T::Meta;
    type Content = T::Meta;
}

/// Defines a processing phase for a document tree.
///
/// This trait specifies all associated types that can be attached as metadata
/// to each node kind in the document tree. Each phase can define its own
/// metadata types to associate with nodes during processing.
pub trait Phase: 'static + Debug + Copy {
    /// Metadata for error nodes
    type Error: Debug + Clone;
    /// Metadata for tag nodes
    type Tag: Debug + Clone;
    /// Metadata for text nodes
    type Text: Debug + Clone;
    /// Metadata for label nodes
    type Label: Debug + Clone;

    // Block
    /// Metadata for block nodes
    type Block: Debug + Clone;
    /// Metadata for raw block nodes
    type Raw: Debug + Clone;
    /// Metadata for heading nodes
    type Heading: Debug + Clone;
    /// Metadata for heading marker nodes
    type HeadingMarker: Debug + Clone;
    /// Metadata for table nodes
    type Table: Debug + Clone;
    /// Metadata for table row nodes
    type TableRow: Debug + Clone;
    /// Metadata for list nodes
    type List: Debug + Clone;
    /// Metadata for list item nodes
    type ListItem: Debug + Clone;
    /// Metadata for enumeration nodes
    type Enum: Debug + Clone;
    /// Metadata for enumeration item nodes
    type EnumItem: Debug + Clone;
    /// Metadata for terms nodes
    type Terms: Debug + Clone;
    /// Metadata for term item nodes
    type TermItem: Debug + Clone;
    /// Metadata for paragraph nodes
    type Paragraph: Debug + Clone;
    /// Metadata for plain text block nodes
    type Plain: Debug + Clone;

    // Inline
    /// Metadata for inline nodes
    type Inline: Debug + Clone;
    /// Metadata for quote nodes
    type Quote: Debug + Clone;
    /// Metadata for strikeout nodes
    type Strikeout: Debug + Clone;
    /// Metadata for emphasis nodes
    type Emphasis: Debug + Clone;
    /// Metadata for strong emphasis nodes
    type Strong: Debug + Clone;
    /// Metadata for subscript nodes
    type Subscript: Debug + Clone;
    /// Metadata for superscript nodes
    type Supscript: Debug + Clone;
    /// Metadata for link nodes
    type Link: Debug + Clone;
    /// Metadata for reference nodes
    type Ref: Debug + Clone;
    /// Metadata for raw inline nodes
    type RawInline: Debug + Clone;
    /// Metadata for inline math nodes
    type MathInline: Debug + Clone;
    /// Metadata for comment nodes
    type Comment: Debug + Clone;
    /// Metadata for escape sequence nodes
    type Escape: Debug + Clone;
    /// Metadata for word nodes
    type Word: Debug + Clone;
    /// Metadata for spacing nodes
    type Spacing: Debug + Clone;
    /// Metadata for soft break nodes
    type SoftBreak: Debug + Clone;

    // Code
    /// Metadata for code nodes
    type Code: Debug + Clone;
    /// Metadata for expression nodes
    type Expr: Debug + Clone;
    /// Metadata for let binding nodes
    type Let: Debug + Clone;
    /// Metadata for bind nodes
    type Bind: Debug + Clone;
    /// Metadata for if expression nodes
    type If: Debug + Clone;
    /// Metadata for for loop nodes
    type For: Debug + Clone;
    /// Metadata for function call nodes
    type Call: Debug + Clone;
    /// Metadata for function arguments nodes
    type Args: Debug + Clone;
    /// Metadata for single argument nodes
    type Arg: Debug + Clone;
    /// Metadata for literal value nodes
    type Literal: Debug + Clone;
    /// Metadata for identifier nodes
    type Ident: Debug + Clone;
    /// Metadata for content nodes
    type Content: Debug + Clone;
}

/// Provides type-safe casting between node-specific metadata and the generic `Meta` enum.
///
/// This trait enables conversion between specific node metadata types and the generic
/// metadata representation, allowing for type-safe metadata manipulation.
pub trait MetaCast<P: Phase> {
    /// The specific metadata type associated with this node type
    type Meta;

    /// Convert a specific metadata type to the generic `Meta` enum
    fn upcast(this: Self::Meta) -> Meta<P>;

    /// Try to extract the specific metadata from a generic `Meta` enum
    fn try_downcast(meta: Meta<P>) -> Option<Self::Meta>;

    /// Try to get a reference to specific metadata from a reference to a generic `Meta` enum
    fn try_downcast_ref(meta: &Meta<P>) -> Option<&Self::Meta>;

    /// Try to get a mutable reference to specific metadata from a mutable reference to a generic `Meta` enum
    fn try_downcast_mut(meta: &mut Meta<P>) -> Option<&mut Self::Meta>;
}

/// Implements the `MetaCast` trait for a list of node types.
///
/// This macro generates implementations of `MetaCast` for all the specified node types,
/// avoiding repetitive boilerplate code.
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

// Implement MetaCast for all node types in the AST
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

/// A generic container for node metadata in a specific processing phase.
///
/// This enum can hold metadata for any node type in the document tree,
/// providing a unified type for metadata storage while maintaining type safety.
#[derive(Clone, Debug)]
pub enum Meta<P: Phase> {
    /// Error node metadata
    Error(<Error as MetaCast<P>>::Meta),
    /// Tag node metadata
    Tag(<Tag as MetaCast<P>>::Meta),
    /// Text node metadata
    Text(<Text as MetaCast<P>>::Meta),
    /// Label node metadata
    Label(<Label as MetaCast<P>>::Meta),

    // Block
    /// Block node metadata
    Block(<Block as MetaCast<P>>::Meta),
    /// Raw block node metadata
    Raw(<Raw as MetaCast<P>>::Meta),
    /// Heading node metadata
    Heading(<Heading as MetaCast<P>>::Meta),
    /// Heading marker node metadata
    HeadingMarker(<HeadingMarker as MetaCast<P>>::Meta),
    /// Table node metadata
    Table(<Table as MetaCast<P>>::Meta),
    /// Table row node metadata
    TableRow(<TableRow as MetaCast<P>>::Meta),
    /// List node metadata
    List(<List as MetaCast<P>>::Meta),
    /// List item node metadata
    ListItem(<ListItem as MetaCast<P>>::Meta),
    /// Enumeration node metadata
    Enum(<Enum as MetaCast<P>>::Meta),
    /// Enumeration item node metadata
    EnumItem(<EnumItem as MetaCast<P>>::Meta),
    /// Terms node metadata
    Terms(<Terms as MetaCast<P>>::Meta),
    /// Term item node metadata
    TermItem(<TermItem as MetaCast<P>>::Meta),
    /// Paragraph node metadata
    Paragraph(<Paragraph as MetaCast<P>>::Meta),
    /// Plain text block node metadata
    Plain(<Plain as MetaCast<P>>::Meta),

    // Inline
    /// Inline node metadata
    Inline(<Inline as MetaCast<P>>::Meta),
    /// Quote node metadata
    Quote(<Quote as MetaCast<P>>::Meta),
    /// Strikeout node metadata
    Strikeout(<Strikeout as MetaCast<P>>::Meta),
    /// Emphasis node metadata
    Emphasis(<Emphasis as MetaCast<P>>::Meta),
    /// Strong emphasis node metadata
    Strong(<Strong as MetaCast<P>>::Meta),
    /// Subscript node metadata
    Subscript(<Subscript as MetaCast<P>>::Meta),
    /// Superscript node metadata
    Supscript(<Supscript as MetaCast<P>>::Meta),
    /// Link node metadata
    Link(<Link as MetaCast<P>>::Meta),
    /// Reference node metadata
    Ref(<Ref as MetaCast<P>>::Meta),
    /// Raw inline node metadata
    RawInline(<RawInline as MetaCast<P>>::Meta),
    /// Inline math node metadata
    MathInline(<MathInline as MetaCast<P>>::Meta),
    /// Comment node metadata
    Comment(<Comment as MetaCast<P>>::Meta),
    /// Escape sequence node metadata
    Escape(<Escape as MetaCast<P>>::Meta),
    /// Word node metadata
    Word(<Word as MetaCast<P>>::Meta),
    /// Spacing node metadata
    Spacing(<Spacing as MetaCast<P>>::Meta),
    /// Soft break node metadata
    SoftBreak(<SoftBreak as MetaCast<P>>::Meta),

    // Code
    /// Code node metadata
    Code(<Code as MetaCast<P>>::Meta),
    /// Expression node metadata
    Expr(<Expr as MetaCast<P>>::Meta),
    /// Let binding node metadata
    Let(<Let as MetaCast<P>>::Meta),
    /// Bind node metadata
    Bind(<Bind as MetaCast<P>>::Meta),
    /// If expression node metadata
    If(<If as MetaCast<P>>::Meta),
    /// For loop node metadata
    For(<For as MetaCast<P>>::Meta),
    /// Function call node metadata
    Call(<Call as MetaCast<P>>::Meta),
    /// Function arguments node metadata
    Args(<Args as MetaCast<P>>::Meta),
    /// Single argument node metadata
    Arg(<Arg as MetaCast<P>>::Meta),
    /// Literal value node metadata
    Literal(<Literal as MetaCast<P>>::Meta),
    /// Identifier node metadata
    Ident(<Ident as MetaCast<P>>::Meta),
    /// Content node metadata
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
    /// Returns a copy of the inner metadata value.
    ///
    /// This method is useful when all metadata types for a phase are the same type
    /// and you need to get a copy of the inner value regardless of the variant.
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

    /// Returns a reference to the inner metadata value.
    ///
    /// This method is useful when all metadata types for a phase are the same type
    /// and you need to get a reference to the inner value regardless of the variant.
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

    /// Returns a mutable reference to the inner metadata value.
    ///
    /// This method is useful when all metadata types for a phase are the same type
    /// and you need to get a mutable reference to the inner value regardless of the variant.
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

/// A wrapper for metadata containers that provides thread-safe sharing.
///
/// This struct wraps a metadata container in an `Arc` for efficient
/// sharing across threads, with type parameters for the processing phase
/// and the specific container type.
#[derive(Debug, Clone)]
pub struct Metadata<P, C = MetaVec<P>>
where
    P: Phase,
    C: MetaContainer<P>,
{
    /// The thread-safe shared metadata container
    container: Arc<C>,
    /// Phantom data to track the phase type parameter
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

/// A vector-based metadata container.
///
/// This type alias represents a simple vector of metadata entries,
/// indexed by node IDs.
pub type MetaVec<P> = Vec<Meta<P>>;

/// Defines operations for storing and retrieving node metadata.
///
/// This trait provides a common interface for different metadata storage
/// implementations, allowing for flexible metadata management strategies.
pub trait MetaContainer<P: Phase>: Sized {
    /// Get a reference to the metadata for a node
    fn get<T>(&self, id: NodeId<T>) -> &Meta<P>;

    /// Get a mutable reference to the metadata for a node
    fn get_mut<T>(&mut self, id: NodeId<T>) -> &mut Meta<P>;

    /// Insert metadata for a node
    fn insert_meta<T>(&mut self, id: NodeId<T>, meta: T::Meta)
    where
        T: MetaCast<P>;

    /// Get a type-specific reference to the metadata for a node
    fn meta<T>(&self, id: NodeId<T>) -> &T::Meta
    where
        T: MetaCast<P>,
    {
        let meta = self.get(id);
        T::try_downcast_ref(meta).unwrap()
    }

    /// Get a type-specific mutable reference to the metadata for a node
    fn meta_mut<T>(&mut self, id: NodeId<T>) -> &mut T::Meta
    where
        T: MetaCast<P>,
    {
        let meta = self.get_mut(id);
        T::try_downcast_mut(meta).unwrap()
    }

    /// Update the metadata for a node and return the old value
    fn update_meta<T>(&mut self, id: NodeId<T>, meta: T::Meta) -> T::Meta
    where
        T: MetaCast<P>,
    {
        std::mem::replace(self.meta_mut(id), meta)
    }
}

/// Implementation of `MetaContainer` for vector-based storage.
///
/// This implementation uses a vector to store metadata, with node IDs directly
/// mapping to vector indices for efficient access.
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

/// A cache-based metadata container that lazily stores metadata.
///
/// This container optimizes for scenarios where not all nodes may need
/// metadata, using `Option` wrappers to indicate presence or absence.
#[derive(Debug, Clone)]
pub struct MetaCache<P: Phase>(Vec<Option<Meta<P>>>);

impl<P: Phase> MetaCache<P> {
    /// Create a new metadata cache with the specified capacity
    pub fn new(size: usize) -> Self {
        Self(vec![None; size])
    }

    /// Extract and take ownership of metadata for a node, removing it from the cache
    pub fn take<T>(&mut self, id: NodeId<T>) -> T::Meta
    where
        T: MetaCast<P>,
    {
        let meta = self.0[id.as_usize()].take().unwrap();

        T::try_downcast(meta).unwrap()
    }
}

/// Implementation of `MetaContainer` for cache-based storage.
///
/// This implementation allows for sparse metadata storage, where entries
/// can be absent (None) when not needed.
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
