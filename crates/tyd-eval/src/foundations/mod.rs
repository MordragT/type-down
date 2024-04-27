use ecow::EcoString;
use std::{collections::BTreeMap, sync::Arc};

use crate::value::Value;

mod args;
mod func;
mod signature;

pub use args::*;
pub use func::*;
pub use signature::*;

pub type Map<E> = Arc<BTreeMap<EcoString, Value<E>>>;
pub type List<E> = Arc<Vec<Value<E>>>;
