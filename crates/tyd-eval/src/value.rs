use derive_more::From;
use ecow::EcoString;
use std::fmt::{self, Debug};
use tyd_syntax::Span;

use crate::{
    error::{ArgumentError, TypeError},
    ir,
    scope::Scope,
    stack::Stack,
    tracer::Tracer,
};

/// Represents a type in the type system used for checking.
///
/// The `Type` enum defines all supported data types in the language,
/// including primitive types, collections, and special types for
/// content handling.
#[derive(Debug, Clone, PartialOrd, Ord, Hash)]
pub enum Type {
    /// A dictionary/object type with named fields of specific types.
    /// Stored as a vector of name-type pairs.
    Map(Vec<(EcoString, Self)>),

    /// A homogeneous collection type that contains elements of a single type.
    List(Box<Self>),

    /// Boolean type representing true/false values.
    Bool,

    /// String type for text data.
    Str,

    /// Floating-point number type.
    Float,

    /// Integer number type.
    Int,

    /// Represents inline content.
    Inline,

    /// Represents block-level content.
    Block,

    /// Generic content type that can hold various forms of content.
    Content,

    /// Special type that can match any other type (used for type checking).
    Any,

    /// Represents the absence of a value or an empty type.
    None,

    /// Function type.
    Func,
}

impl Type {
    /// Creates a new list type containing elements of the specified type.
    ///
    /// # Arguments
    ///
    /// * `ty` - The type of elements the list will contain
    ///
    /// # Returns
    ///
    /// A new `Type::List` variant containing the specified element type
    pub fn list(ty: Self) -> Self {
        Self::List(Box::new(ty))
    }
}

impl fmt::Display for Type {
    /// Formats the type for display purposes.
    ///
    /// This implementation provides a human-readable representation of each type variant.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Map(map) => {
                write!(f, "Map {{")?;
                for (name, ty) in map {
                    write!(f, "{name}: {ty}")?;
                }
                write!(f, "}}")
            }
            Type::List(ty) => write!(f, "List {ty}"),
            Type::Bool => write!(f, "Bool"),
            Type::Str => write!(f, "Str"),
            Type::Float => write!(f, "Float"),
            Type::Int => write!(f, "Int"),
            Type::Inline => write!(f, "Inline"),
            Type::Block => write!(f, "Block"),
            Type::Content => write!(f, "Content"),
            Type::Any => write!(f, "Any"),
            Type::None => write!(f, "None"),
            Type::Func => write!(f, "Func"),
        }
    }
}

impl PartialEq for Type {
    /// Compares two types for equality.
    ///
    /// Note that `Type::Any` is considered equal to any other type,
    /// making it behave as a special wildcard type for compatibility checks.
    fn eq(&self, other: &Self) -> bool {
        use Type::*;

        match (self, other) {
            (Map(x), Map(y)) => x == y,
            (List(x), List(y)) => x == y,
            (Bool, Bool) => true,
            (Str, Str) => true,
            (Float, Float) => true,
            (Int, Int) => true,
            (Inline, Inline) => true,
            (Block, Block) => true,
            (Content, Content) => true,
            (Func, Func) => true,
            (Any, _) => true,
            (_, Any) => true,
            _ => false,
        }
    }
}

impl Eq for Type {}

/// Represents a runtime value in the language.
///
/// This enum encompasses all possible value types that can be used in the system,
/// from primitive types like integers and strings to complex structures like maps and functions.
#[derive(Debug, Clone, From)]
pub enum Value {
    /// A key-value mapping structure
    Map(ir::Map),
    /// An ordered collection of values
    List(ir::List),
    /// A boolean value (true or false)
    Bool(bool),
    /// A string value
    Str(EcoString),
    /// A 64-bit floating point number
    Float(f64),
    /// A 64-bit signed integer
    Int(i64),
    /// An inline element
    Inline(ir::Inline),
    /// A block element
    Block(ir::Block),
    /// A content element
    Content(ir::Content),
    /// A function value
    Func(ir::Func),
    /// Represents the absence of a value
    None,
}

impl Value {
    /// Returns the type of this value.
    ///
    /// For collections like Map and List, this method computes the appropriate
    /// type based on the collection's content.
    pub fn ty(&self) -> Type {
        match self {
            Self::Map(map) => {
                let inner = map
                    .iter()
                    .map(|(name, val)| (name.clone(), val.ty()))
                    .collect();

                Type::Map(inner)
            }
            Self::List(list) => {
                if list.is_empty() {
                    Type::list(Type::Any)
                } else {
                    Type::list(list.first().unwrap().ty())
                }
            }
            Self::Content(_) => Type::Content,
            Self::Bool(_) => Type::Bool,
            Self::Str(_) => Type::Str,
            Self::Float(_) => Type::Float,
            Self::Int(_) => Type::Int,
            Self::Inline(_) => Type::Inline,
            Self::Block(_) => Type::Block,
            Self::Func(_) => Type::Func,
            Self::None => Type::None,
        }
    }
}

/// Implements conversion from string literals to Value.
impl From<&str> for Value {
    /// Converts a string literal into a Value::Str.
    fn from(value: &str) -> Self {
        Value::Str(value.into())
    }
}

/// A helper structure for type checking and extraction of values.
///
/// This struct simplifies the process of extracting typed values from scopes and stacks,
/// automatically handling error reporting through the tracer.
pub struct TypeChecker<'a> {
    /// Reference to the error tracer for reporting type errors
    tracer: &'a mut Tracer,
    /// The source span where type checking is being performed
    span: Span,
}

impl<'a> TypeChecker<'a> {
    /// Creates a new TypeChecker with the given tracer and span.
    ///
    /// # Arguments
    ///
    /// * `tracer` - Mutable reference to a Tracer to report errors
    /// * `span` - The source span associated with the current operation
    pub fn new(tracer: &'a mut Tracer, span: Span) -> Self {
        Self { tracer, span }
    }

    /// Extracts a value from a Result, reporting a type error if the result is an error.
    ///
    /// # Arguments
    ///
    /// * `value` - A Result containing either the successfully typed value or an error type
    ///
    /// # Returns
    ///
    /// * `Some(val)` if the value was of the correct type
    /// * `None` if there was a type error (and reports the error via the tracer)
    pub fn extract<T>(&mut self, value: Result<T, Type>) -> Option<T>
    where
        T: Typed,
    {
        match value {
            Ok(val) => Some(val),
            Err(got) => {
                self.tracer.source_error(
                    self.span,
                    TypeError::WrongType {
                        got,
                        expected: T::type_name(),
                    },
                );
                None
            }
        }
    }

    /// Removes and extracts a typed value from a scope by name.
    ///
    /// Reports appropriate errors if the value is missing or of the wrong type.
    ///
    /// # Arguments
    ///
    /// * `scope` - Mutable reference to the scope
    /// * `name` - The name of the value to extract
    ///
    /// # Returns
    ///
    /// * `Some(val)` if the value was found with the correct type
    /// * `None` if the value was missing or had the wrong type (and reports the error)
    pub fn remove_from_scope<T>(&mut self, scope: &mut Scope, name: impl AsRef<str>) -> Option<T>
    where
        T: Typed,
    {
        let name = name.as_ref();

        match scope.try_remove::<T>(name) {
            Some(result) => self.extract(result),
            None => {
                self.tracer.source_error(
                    self.span,
                    ArgumentError::MissingRequired {
                        name: name.into(),
                        ty: T::type_name(),
                    },
                );
                None
            }
        }
    }

    /// Removes and extracts a typed value from a scope by name, or returns a default value if not found.
    ///
    /// This method attempts to extract a value of type T from the scope using the given name.
    /// If the value is present but of the wrong type, it reports an error via the tracer.
    /// If the value is not present at all, it returns the provided default value.
    ///
    /// # Arguments
    ///
    /// * `scope` - Mutable reference to the scope
    /// * `name` - The name of the value to extract
    /// * `default` - The default value to return if the named value is not found
    ///
    /// # Returns
    ///
    /// * `Some(val)` if the value was found with the correct type or the default value was used
    /// * `None` if the value was found but had the wrong type (and reports the error)
    pub fn remove_from_scope_or<T>(
        &mut self,
        scope: &mut Scope,
        name: impl AsRef<str>,
        default: T,
    ) -> Option<T>
    where
        T: Typed,
    {
        match scope.try_remove::<T>(name) {
            Some(result) => self.extract(result),
            None => Some(default),
        }
    }

    /// Pops and extracts a typed value from a stack at the given position.
    ///
    /// Reports appropriate errors if the value is missing or of the wrong type.
    ///
    /// # Arguments
    ///
    /// * `stack` - Mutable reference to the stack
    /// * `pos` - The position of the argument in the stack
    ///
    /// # Returns
    ///
    /// * `Some(val)` if the value was popped with the correct type
    /// * `None` if the value was missing or had the wrong type (and reports the error)
    pub fn pop_from_stack<T>(&mut self, stack: &mut Stack, pos: usize) -> Option<T>
    where
        T: Typed,
    {
        match stack.try_pop::<T>() {
            Some(result) => self.extract(result),
            None => {
                self.tracer.source_error(
                    self.span,
                    ArgumentError::MissingPositional {
                        pos,
                        ty: T::type_name(),
                    },
                );
                None
            }
        }
    }

    pub fn pop_from_stack_or<T>(&mut self, stack: &mut Stack, default: T) -> Option<T>
    where
        T: Typed,
    {
        match stack.try_pop::<T>() {
            Some(value) => self.extract(value),
            None => Some(default),
        }
    }

    /// Warns about unknown positional arguments in a stack.
    ///
    /// This method generates warnings for any remaining positional arguments
    /// in the stack that were not consumed by the function.
    ///
    /// # Arguments
    ///
    /// * `stack` - The stack containing remaining arguments
    /// * `start_pos` - The starting position for numbering arguments (usually
    ///   the number of arguments already consumed)
    pub fn warn_unknown_positional(&mut self, stack: Stack, start_pos: usize) {
        for (pos, _) in stack.into_iter().enumerate() {
            self.tracer.source_warn(
                self.span,
                ArgumentError::UnknownPositional {
                    pos: pos + start_pos,
                },
            );
        }
    }

    /// Warns about unknown named arguments in a scope.
    ///
    /// This method generates warnings for any remaining named arguments
    /// in the scope that were not consumed by the function.
    ///
    /// # Arguments
    ///
    /// * `scope` - The scope containing remaining named arguments
    pub fn warn_unknown_named(&mut self, scope: Scope) {
        for name in scope.into_symbols() {
            self.tracer
                .source_warn(self.span, ArgumentError::UnknownNamed { name });
        }
    }
}

/// A trait for types that have a runtime type representation.
///
/// This trait extends TypeCast to add type information capabilities,
/// enabling type-aware operations and error reporting.
pub trait Typed: TypeCast {
    /// Returns the Type enum value corresponding to this Rust type.
    fn type_name() -> Type;
}

/// Macro that implements the Typed trait for multiple types.
///
/// This reduces boilerplate by generating the implementation for each
/// specified type, mapping it to the corresponding Type enum variant.
macro_rules! impl_typed {
    ($($variant:ident($variant_type:ty)),*) => {
        $(
            impl Typed for $variant_type {
                fn type_name() -> Type {
                    Type::$variant
                }
            }
        )*
    };
}

// Implement Typed for primitive and IR types
impl_typed!(
    Bool(bool),
    Str(EcoString),
    Float(f64),
    Int(i64),
    Inline(ir::Inline),
    Block(ir::Block),
    Content(ir::Content),
    Func(ir::Func)
);

/// Custom implementation of Typed for List type.
///
/// Lists have a parameterized type, so we use a more general
/// representation with Any as the inner type.
impl Typed for ir::List {
    fn type_name() -> Type {
        Type::list(Type::Any)
    }
}

/// A trait for converting between Value enum and concrete types.
///
/// This trait provides methods for type conversions in both directions:
/// - Converting a concrete type into a Value variant (upcasting)
/// - Attempting to extract a concrete type from a Value (downcasting)
pub trait TypeCast: Sized + Clone {
    /// Converts self into a Value enum variant.
    fn upcast(self) -> Value;

    /// Attempts to extract a concrete type from a Value by consuming it.
    /// Returns an error containing the actual type if the conversion fails.
    fn try_downcast(v: Value) -> Result<Self, Type>;

    /// Attempts to get a reference to the underlying value of a specific type.
    /// Returns an error containing the actual type if the conversion fails.
    fn try_downcast_ref(v: &Value) -> Result<&Self, Type>;

    /// Attempts to get a mutable reference to the underlying value of a specific type.
    /// Returns an error containing the actual type if the conversion fails.
    fn try_downcast_mut(v: &mut Value) -> Result<&mut Self, Type>;

    /// Attempts to clone and extract a value of a specific type from a reference to a Value.
    /// This is a convenience method that combines try_downcast_ref and cloning.
    fn try_downcast_cloned(v: &Value) -> Result<Self, Type> {
        Self::try_downcast_ref(v).cloned()
    }
}

/// Macro that implements TypeCast for a set of types.
///
/// This reduces boilerplate by generating the TypeCast implementation
/// for each specified type, mapping it to the corresponding Value variant.
macro_rules! impl_type_cast {
    ($($variant:ident($variant_type:ty)),*) => {
        $(
            impl TypeCast for $variant_type {
                fn upcast(self) -> Value {
                    Value::$variant(self)
                }

                fn try_downcast(v: Value) -> Result<Self, Type> {
                    match v {
                        Value::$variant(val) => Ok(val),
                        _ => Err(v.ty()),
                    }
                }

                fn try_downcast_ref(v: &Value) -> Result<&Self, Type> {
                    match v {
                        Value::$variant(val) => Ok(val),
                        _ => Err(v.ty()),
                    }
                }

                fn try_downcast_mut(v: &mut Value) -> Result<&mut Self, Type> {
                    match v {
                        Value::$variant(val) => Ok(val),
                        _ => Err(v.ty()),
                    }
                }
            }
        )*
    };
}

// Implement TypeCast for all value variants
impl_type_cast!(
    Map(ir::Map),
    List(ir::List),
    Bool(bool),
    Str(EcoString),
    Float(f64),
    Int(i64),
    Inline(ir::Inline),
    Block(ir::Block),
    Content(ir::Content),
    Func(ir::Func)
);

/// Implementation of TypeCast for Value itself.
///
/// This is an identity implementation that allows Value to be treated
/// as any other type that implements TypeCast.
impl TypeCast for Value {
    fn upcast(self) -> Value {
        self
    }

    fn try_downcast(v: Value) -> Result<Self, Type> {
        Ok(v)
    }

    fn try_downcast_ref(v: &Value) -> Result<&Self, Type> {
        Ok(v)
    }

    fn try_downcast_mut(v: &mut Value) -> Result<&mut Self, Type> {
        Ok(v)
    }
}
