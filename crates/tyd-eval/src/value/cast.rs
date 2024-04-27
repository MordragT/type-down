use ecow::EcoString;

use super::{Map, Value};
use crate::eval::Engine;

pub trait Cast<E: Engine> {
    fn cast(value: Value<E>) -> Self;
}

impl<E: Engine> Cast<E> for Map<E> {
    fn cast(value: Value<E>) -> Self {
        value.into_map().unwrap()
    }
}

impl<E: Engine, T: Cast<E>> Cast<E> for Vec<T> {
    fn cast(value: Value<E>) -> Self {
        let list = value.into_list().unwrap();
        list.iter().cloned().map(T::cast).collect()
    }
}

impl<E: Engine> Cast<E> for bool {
    fn cast(value: Value<E>) -> Self {
        value.into_bool().unwrap()
    }
}

impl<E: Engine> Cast<E> for EcoString {
    fn cast(value: Value<E>) -> Self {
        value.into_string().unwrap()
    }
}

impl<E: Engine> Cast<E> for f64 {
    fn cast(value: Value<E>) -> Self {
        value.into_float().unwrap()
    }
}

impl<E: Engine> Cast<E> for i64 {
    fn cast(value: Value<E>) -> Self {
        value.into_int().unwrap()
    }
}
