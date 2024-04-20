use tyd_syntax::ast::{Args, Block, Call, Expr, Ident, Inline};

use crate::{
    error::{EngineError, EngineErrorHandler, EngineErrorMessage::*, EngineErrors},
    RawArgsBuilder, Shape, Signature, SymbolTable, ValidArgs, Validator, Value,
};

pub trait Engine<S: Shape> {
    type Error: From<EngineError> + From<EngineErrors>;
    type State: SymbolTable<S> + EngineErrorHandler;

    fn eval_inline(
        &self,
        state: &mut Self::State,
        inline: &Inline,
    ) -> Result<S::Inline, Self::Error>;

    fn eval_block(&self, state: &mut Self::State, block: &Block) -> Result<S::Block, Self::Error>;

    fn eval_text(
        &self,
        state: &mut Self::State,
        text: &Vec<Inline>,
    ) -> Result<Value<S>, Self::Error> {
        let mut result = Vec::new();

        for inline in text {
            let evaluated = self.eval_inline(state, inline)?;
            result.push(Value::Inline(evaluated));
        }

        Ok(Value::List(result))
    }

    fn eval_expr(&self, state: &mut Self::State, expr: &Expr) -> Result<Value<S>, Self::Error> {
        match expr {
            Expr::Block(block) => todo!(),
            Expr::Call(call) => self.eval_call(state, call),
            Expr::Ident(ident) => self.eval_symbol(state, ident),
            Expr::Literal(literal) => Ok(Value::from(literal.to_owned())),
        }
    }

    fn eval_symbol(&self, state: &mut Self::State, ident: &Ident) -> Result<Value<S>, Self::Error> {
        let key = &ident.value;

        let value = state.symbol(key).ok_or(EngineError::new(
            ident.span,
            SymbolNotFound(key.to_string()),
        ))?;

        Ok(value)
    }

    fn eval_call(&self, state: &mut Self::State, call: &Call) -> Result<Value<S>, Self::Error> {
        let Call { ident, args, span } = call;

        let key = &ident.value;

        let cmd = state
            .command(key)
            .ok_or(EngineError::new(*span, FunctionNotFound(key.to_string())))?;
        let signature = cmd.signature();

        let mut args = self.eval_args(state, signature, args)?;
        let value = cmd.run(&mut args)?;

        if !args.is_empty() {
            panic!("ERROR: Unused arguments");
        }

        Ok(value)
    }

    fn eval_args(
        &self,
        state: &mut Self::State,
        signature: Signature<S>,
        args: &Args,
    ) -> Result<ValidArgs<S>, Self::Error> {
        let mut raw_args = RawArgsBuilder::new();
        let Args {
            args,
            content,
            span,
        } = args;

        for arg in args {
            // TODO use the position of the arg and a Function trait wich has a method args -> &[&str] where
            // the args are shown in correct order to get the name if not specified.
            let name = arg.name.as_ref().unwrap();
            let value = self.eval_expr(state, &arg.value)?;

            raw_args.insert(name, arg.span, value);
        }

        if let Some(content) = content {
            let value = self.eval_text(state, &content.content)?;
            raw_args.insert("content".to_owned(), content.span, value);
        }
        let result = Validator::new(*span, raw_args.build(), signature).validate();

        if result.has_errors() {
            let error = EngineErrors {
                src: state.named_source(),
                related: result.1,
            };
            Err(error.into())
        } else {
            Ok(result.0)
        }
    }
}
