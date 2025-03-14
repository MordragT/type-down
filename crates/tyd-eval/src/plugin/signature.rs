use ecow::EcoString;

use crate::{eval::Engine, ty::Type, value::Value};

#[derive(Debug, Clone)]
pub struct Signature<E: Engine> {
    pub name: String,
    pub positional: Vec<Type>,
    pub required: Vec<(EcoString, Type)>,
    pub optional: Vec<(EcoString, Value<E>)>,
}

impl<E: Engine> Signature<E> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            positional: Vec::new(),
            required: Vec::new(),
            optional: Vec::new(),
        }
    }

    pub fn names(&self) -> impl Iterator<Item = &EcoString> {
        self.required_names().chain(self.optional_names())
    }

    pub fn required_names(&self) -> impl Iterator<Item = &EcoString> {
        self.required.iter().map(|(n, _)| n)
    }

    pub fn optional_names(&self) -> impl Iterator<Item = &EcoString> {
        self.optional.iter().map(|(n, _)| n)
    }

    pub fn positonal_count(&self) -> usize {
        self.positional.len()
    }

    pub fn get_required(&self, name: impl AsRef<str>) -> Option<Type> {
        self.required.iter().find_map(|(n, ty)| {
            if n == name.as_ref() {
                Some(ty.clone())
            } else {
                None
            }
        })
    }

    pub fn get_optional(&self, name: impl AsRef<str>) -> Option<Type> {
        self.optional.iter().find_map(|(n, val)| {
            if n == name.as_ref() {
                Some(val.ty())
            } else {
                None
            }
        })
    }

    pub fn get_default(&self, name: impl AsRef<str>) -> Option<&Value<E>> {
        self.optional
            .iter()
            .find_map(|(n, val)| if n == name.as_ref() { Some(val) } else { None })
    }

    pub fn get_positional(&self, position: usize) -> Option<Type> {
        self.positional.get(position).cloned()
    }

    pub fn contains(&self, name: impl AsRef<str>) -> bool {
        let name = name.as_ref();

        self.required.iter().find(|(n, _)| n == name).is_some()
            || self.optional.iter().find(|(n, _)| n == name).is_some()
    }

    pub fn positional(mut self, ty: Type) -> Self {
        self.positional.push(ty);
        self
    }

    pub fn optional(mut self, name: impl Into<EcoString>, default: impl Into<Value<E>>) -> Self {
        self.optional.push((name.into(), default.into()));
        self
    }

    pub fn required(mut self, name: impl Into<EcoString>, ty: Type) -> Self {
        self.required.push((name.into(), ty));
        self
    }
}
