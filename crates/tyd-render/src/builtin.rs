use crate::{error::EngineError, Args, Value};

pub fn list<C>(args: Args<C>) -> Result<Value<C>, EngineError> {
    let list = args.into_values().collect();

    Ok(Value::List(list))
}

pub fn dict<C>(args: Args<C>) -> Result<Value<C>, EngineError> {
    Ok(Value::Map(args))
}
