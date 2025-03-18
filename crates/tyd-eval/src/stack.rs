use std::{mem, slice, vec};

use crate::{
    ty::Type,
    value::{TypeCast, Value},
};

#[derive(Debug, Clone)]
pub struct Stack(Vec<Value>);

impl Stack {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn take(&mut self) -> Stack {
        Stack(mem::take(&mut self.0))
    }

    #[inline]
    pub fn replace(&mut self, src: Stack) -> Stack {
        Stack(mem::replace(&mut self.0, src.0))
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Value> {
        self.0.pop()
    }

    #[inline]
    pub fn try_pop<T>(&mut self) -> Option<Result<T, Type>>
    where
        T: TypeCast,
    {
        self.pop().map(T::try_downcast)
    }

    #[inline]
    pub fn push(&mut self, v: Value) {
        self.0.push(v)
    }

    #[inline]
    pub fn push_none(&mut self) {
        self.0.push(Value::None)
    }

    #[inline]
    pub fn iter(&self) -> slice::Iter<Value> {
        self.0.iter()
    }

    #[inline]
    pub fn into_inner(self) -> Vec<Value> {
        self.0
    }
}

impl IntoIterator for Stack {
    type IntoIter = vec::IntoIter<Value>;
    type Item = Value;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
