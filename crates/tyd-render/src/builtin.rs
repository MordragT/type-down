use crate::{error::ContextError, Args, Value};

pub fn list<C>(args: Args<C>) -> Result<Value<C>, ContextError> {
    let list = args.into_values().collect();

    Ok(Value::List(list))
}

pub fn dict<C>(args: Args<C>) -> Result<Value<C>, ContextError> {
    Ok(Value::Map(args))
}
