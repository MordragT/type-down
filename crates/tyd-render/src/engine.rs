use tyd_syntax::ast;

use crate::{
    error::{EngineError, EngineErrorHandler, EngineErrorMessage::*, EngineErrors},
    Arg, Shape, SymbolTable, Value,
};

pub trait Engine<S: Shape> {
    type Error: From<EngineError> + From<EngineErrors>;
    type State: SymbolTable<S> + EngineErrorHandler;

    fn eval_inline(
        &self,
        state: &mut Self::State,
        inline: &ast::Inline,
    ) -> Result<S::Inline, Self::Error>;

    fn eval_block(
        &self,
        state: &mut Self::State,
        block: &ast::Block,
    ) -> Result<S::Block, Self::Error>;

    fn eval_text(
        &self,
        state: &mut Self::State,
        text: &Vec<ast::Inline>,
    ) -> Result<Value<S>, Self::Error> {
        let mut result = Vec::new();

        for inline in text {
            let evaluated = self.eval_inline(state, inline)?;
            result.push(Value::Inline(evaluated));
        }

        Ok(Value::List(result))
    }

    fn eval_expr(
        &self,
        state: &mut Self::State,
        expr: &ast::Expr,
    ) -> Result<Value<S>, Self::Error> {
        match expr {
            ast::Expr::Block(block) => todo!(),
            ast::Expr::Call(call) => self.eval_call(state, call),
            ast::Expr::Ident(ident) => self.eval_symbol(state, ident),
            ast::Expr::Literal(literal) => Ok(Value::from(literal.to_owned())),
        }
    }

    fn eval_symbol(
        &self,
        state: &mut Self::State,
        ident: &ast::Ident,
    ) -> Result<Value<S>, Self::Error> {
        let key = &ident.value;

        let value = state.symbol(key).ok_or(EngineError::new(
            ident.span,
            SymbolNotFound(key.to_string()),
        ))?;

        Ok(value)
    }

    fn eval_call(
        &self,
        state: &mut Self::State,
        call: &ast::Call,
    ) -> Result<Value<S>, Self::Error> {
        let ast::Call { ident, args, span } = call;

        let key = &ident.value;
        let cmd = state
            .command(key)
            .ok_or(EngineError::new(*span, FunctionNotFound(key.to_string())))?;

        let args = self.eval_args(state, args)?;
        let value = cmd.dispatch(args, *span).map_err(|related| EngineErrors {
            src: state.named_source(),
            related,
        })?;

        Ok(value)
    }

    fn eval_args(
        &self,
        state: &mut Self::State,
        args: &ast::Args,
    ) -> Result<Vec<Arg<S>>, Self::Error> {
        let ast::Args {
            args,
            content,
            span: _,
        } = args;

        let mut args = args
            .iter()
            .map(|arg| self.eval_arg(state, arg))
            .collect::<Result<Vec<_>, Self::Error>>()?;

        if let Some(ast::Content { content, span }) = content {
            let value = self.eval_text(state, content)?;

            args.push(Arg {
                name: Some("content".to_owned()),
                span: *span,
                value,
            });
        }

        Ok(args)
    }

    fn eval_arg(&self, state: &mut Self::State, arg: &ast::Arg) -> Result<Arg<S>, Self::Error> {
        let ast::Arg { name, value, span } = arg;

        let value = self.eval_expr(state, value)?;

        Ok(Arg {
            name: name.as_ref().map(|s| s.to_string()),
            span: *span,
            value,
        })
    }
}
