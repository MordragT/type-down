use std::collections::BTreeMap;

use crate::{Shape, Type, Value};

#[derive(Debug, Clone)]
pub struct Signature<S: Shape> {
    pub name: String,
    pub params: BTreeMap<String, Parameter<S>>,
}

impl<S: Shape> Signature<S> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            params: BTreeMap::new(),
        }
    }

    pub fn optional(
        mut self,
        name: impl Into<String>,
        ty: Type,
        default: impl Into<Value<S>>,
    ) -> Self {
        self.params
            .insert(name.into(), Parameter::optional(ty, default));
        self
    }

    pub fn required(mut self, name: impl Into<String>, ty: Type) -> Self {
        self.params.insert(name.into(), Parameter::required(ty));
        self
    }

    pub fn insert(mut self, name: impl Into<String>, param: Parameter<S>) -> Self {
        self.params.insert(name.into(), param);
        self
    }
}

#[derive(Debug, Clone)]
pub struct Parameter<S: Shape> {
    pub ty: Type,
    pub default: Option<Value<S>>,
}

impl<S: Shape> Parameter<S> {
    #[inline]
    pub fn required(ty: Type) -> Self {
        Self { ty, default: None }
    }

    #[inline]
    pub fn optional(ty: Type, default: impl Into<Value<S>>) -> Self {
        Self {
            ty,
            default: Some(default.into()),
        }
    }
}
