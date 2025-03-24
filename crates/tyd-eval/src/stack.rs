use std::{mem, slice, vec};

use crate::value::{Type, TypeCast, Value};

/// A stack implementation for storing `Value` objects.
///
/// This data structure provides LIFO (Last-In-First-Out) operations
/// and is used for managing values during execution.
#[derive(Debug, Clone)]
pub struct Stack(Vec<Value>);

impl Stack {
    /// Creates a new empty stack.
    ///
    /// # Returns
    ///
    /// A new, empty `Stack` instance.
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Removes all elements from the stack.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Checks if the stack is empty.
    ///
    /// # Returns
    ///
    /// `true` if the stack contains no elements, `false` otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Takes all elements from the stack, leaving it empty.
    ///
    /// # Returns
    ///
    /// A new `Stack` containing all elements that were in this stack.
    #[inline]
    pub fn take(&mut self) -> Stack {
        Stack(mem::take(&mut self.0))
    }

    /// Replaces the contents of this stack with the contents of another stack.
    ///
    /// # Parameters
    ///
    /// - `src`: The stack whose contents will replace this stack's contents.
    ///
    /// # Returns
    ///
    /// A `Stack` containing the original contents of this stack.
    #[inline]
    pub fn replace(&mut self, src: Stack) -> Stack {
        Stack(mem::replace(&mut self.0, src.0))
    }

    /// Removes and returns the top element from the stack.
    ///
    /// # Returns
    ///
    /// `Some(Value)` if the stack is not empty, `None` otherwise.
    #[inline]
    pub fn pop(&mut self) -> Option<Value> {
        self.0.pop()
    }

    /// Attempts to pop and downcast a value from the stack to a specific type.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type to convert the popped value to. Must implement `TypeCast`.
    ///
    /// # Returns
    ///
    /// - `None` if the stack is empty
    /// - `Some(Ok(T))` if conversion succeeded
    /// - `Some(Err(Type))` if conversion failed, containing the actual type
    #[inline]
    pub fn try_pop<T>(&mut self) -> Option<Result<T, Type>>
    where
        T: TypeCast,
    {
        self.pop().map(T::try_downcast)
    }

    /// Pushes a value onto the stack.
    ///
    /// # Parameters
    ///
    /// - `v`: The value to push onto the stack.
    #[inline]
    pub fn push(&mut self, v: Value) {
        self.0.push(v)
    }

    /// Pushes a `None` value onto the stack.
    #[inline]
    pub fn push_none(&mut self) {
        self.0.push(Value::None)
    }

    /// Returns an iterator over the stack's elements.
    ///
    /// # Returns
    ///
    /// An iterator yielding references to each `Value` in the stack.
    #[inline]
    pub fn iter(&self) -> slice::Iter<Value> {
        self.0.iter()
    }

    /// Consumes the stack and returns the underlying vector.
    ///
    /// # Returns
    ///
    /// The vector containing all the stack's elements.
    #[inline]
    pub fn into_inner(self) -> Vec<Value> {
        self.0
    }
}

/// Implements the `IntoIterator` trait to allow consuming the stack as an iterator.
impl IntoIterator for Stack {
    type IntoIter = vec::IntoIter<Value>;
    type Item = Value;

    /// Consumes the stack and returns an iterator over its elements.
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
