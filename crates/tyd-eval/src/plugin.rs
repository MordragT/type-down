use std::collections::{BTreeMap, HashSet};

use ecow::EcoString;
use tyd_syntax::Span;

use crate::{error::*, ir::Arguments, tracer::Tracer, ty::Type, value::Value};

pub trait Plugin {
    fn signature() -> Signature;
    fn call(args: Arguments, tracer: &mut Tracer) -> Value;
}

pub fn dispatch<P>(args: Arguments, tracer: &mut Tracer) -> Value
where
    P: Plugin,
{
    let signature = P::signature();

    let args = match signature.validate(args) {
        Ok(args) => args,
        Err(errs) => {
            tracer.errors(errs);
            return Value::None;
        }
    };

    P::call(args, tracer)
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub name: String,
    pub positional: Vec<Type>,
    pub required: Vec<(EcoString, Type)>,
    pub optional: Vec<(EcoString, Value)>,
}

impl Signature {
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

    pub fn get_default(&self, name: impl AsRef<str>) -> Option<&Value> {
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

    pub fn optional(mut self, name: impl Into<EcoString>, default: impl Into<Value>) -> Self {
        self.optional.push((name.into(), default.into()));
        self
    }

    pub fn required(mut self, name: impl Into<EcoString>, ty: Type) -> Self {
        self.required.push((name.into(), ty));
        self
    }
}

impl Signature {
    pub fn validate(&self, args: Arguments) -> Result<Arguments, Vec<EngineError>> {
        use ArgumentError::*;

        let Arguments {
            mut named,
            positional,
            span,
            source,
        } = args;

        let mut errors = Vec::new();

        for (name, value) in named.iter() {
            if let Err(e) = self.validate_named(name, value, span) {
                errors.push(e);
            }
        }

        for (pos, value) in positional.iter().enumerate() {
            if let Err(e) = self.validate_positional(pos, &value, span) {
                errors.push(e);
            }
        }

        let gotten = named.keys().collect::<HashSet<_>>();
        let required = self.required_names().collect::<HashSet<_>>();

        for name in required.difference(&gotten) {
            let ty = self.get_required(name).unwrap();

            errors.push(EngineError::arg(
                span,
                MissingRequired {
                    name: (**name).clone(),
                    ty,
                },
            ))
        }

        for pos in positional.len()..self.positonal_count() {
            let ty = self.get_positional(pos).unwrap();

            errors.push(EngineError::arg(span, MissingPositional { pos, ty }))
        }

        let optional = self.optional_names().collect::<HashSet<_>>();

        let mut defaults = optional
            .difference(&gotten)
            .map(|name| {
                let name = (**name).clone();
                let value = self.get_default(&name).unwrap().to_owned();
                (name, value)
            })
            .collect::<BTreeMap<_, _>>();

        named.append(&mut defaults);

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Arguments {
                named,
                positional,
                span,
                source,
            })
        }
    }

    fn validate_named(
        &self,
        name: &EcoString,
        value: &Value,
        span: Span,
    ) -> Result<(), EngineError> {
        use ArgumentError::*;

        let ty = self
            .get_required(&name)
            .or(self.get_optional(&name))
            .ok_or(EngineError::arg(span, UnknownNamed { name: name.clone() }))?;

        let got = value.ty();

        if ty == got {
            Ok(())
        } else {
            Err(EngineError::arg(span, WrongType { got, expected: ty }))
        }
    }

    fn validate_positional(
        &self,
        pos: usize,
        value: &Value,
        span: Span,
    ) -> Result<(), EngineError> {
        use ArgumentError::*;

        let ty = self
            .get_positional(pos)
            .ok_or(EngineError::arg(span, UnknownPositional { pos }))?;

        let got = value.ty();

        if ty == got {
            Ok(())
        } else {
            Err(EngineError::arg(span, WrongType { got, expected: ty }))
        }
    }
}

// use ecow::EcoString;

// use crate::{
//     eval::{Engine, Scope},
//     hir,
//     value::Value,
// };

// mod func;
// mod signature;

// pub use func::PluginFunc;
// pub use signature::Signature;

// pub struct Plugin<E: Engine> {
//     scope: Scope<E>,
// }

// impl<E: Engine> Plugin<E> {
//     pub fn new() -> Self {
//         Self {
//             scope: Scope::new(),
//         }
//     }

//     pub fn register_symbol<N, V>(mut self, name: N, value: V) -> Self
//     where
//         N: Into<EcoString>,
//         V: Into<Value<E>>,
//     {
//         self.scope.define_symbol(name, value);
//         self
//     }

//     pub fn register_func<F>(mut self, name: impl Into<EcoString>) -> Self
//     where
//         F: PluginFunc<E>,
//     {
//         self.define_func::<F>(name);
//         self
//     }

//     pub fn define_symbol<N, V>(&mut self, name: N, value: V) -> Option<Value<E>>
//     where
//         N: Into<EcoString>,
//         V: Into<Value<E>>,
//     {
//         self.scope.define_symbol(name, value)
//     }

//     pub fn define_func<F>(&mut self, name: impl Into<EcoString>) -> Option<Value<E>>
//     where
//         F: PluginFunc<E>,
//     {
//         let f = func::dispatch::<E, F>;
//         let func = hir::Func::new(f);

//         self.scope.define_func(name, func)
//     }

//     pub fn into_scope(self) -> Scope<E> {
//         self.scope
//     }
// }
