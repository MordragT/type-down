use ecow::EcoString;

use super::{Map, Value};

pub trait Downcast {
    fn downcast(value: Value) -> Self;
}

impl Downcast for Map {
    fn downcast(value: Value) -> Self {
        value.into_map().unwrap()
    }
}

impl<T: Downcast> Downcast for Vec<T> {
    fn downcast(value: Value) -> Self {
        let list = value.into_list().unwrap();
        list.iter().cloned().map(T::downcast).collect()
    }
}

impl Downcast for bool {
    fn downcast(value: Value) -> Self {
        value.into_bool().unwrap()
    }
}

impl Downcast for EcoString {
    fn downcast(value: Value) -> Self {
        value.into_string().unwrap()
    }
}

impl Downcast for f64 {
    fn downcast(value: Value) -> Self {
        value.into_float().unwrap()
    }
}

impl Downcast for i64 {
    fn downcast(value: Value) -> Self {
        value.into_int().unwrap()
    }
}
