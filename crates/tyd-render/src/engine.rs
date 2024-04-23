use tyd_syntax::ast;

use crate::{
    command::{Arg, UnverifiedCall},
    context::{Context, SymbolTable},
    error::{EngineError, EngineMessage::*},
    value::{Shape, Value},
};

pub trait Engine<S: Shape> {
    type State: Context<S>;

    fn eval_inline(&self, state: &mut Self::State, inline: &ast::Inline) -> Option<S::Inline>;

    fn eval_block(&self, state: &mut Self::State, block: &ast::Block) -> Option<S::Block>;

    fn eval_expr(&self, state: &mut Self::State, expr: &ast::Expr) -> Option<Value<S>> {
        match expr {
            ast::Expr::Block(block, _) => todo!(),
            ast::Expr::Call(call) => self.eval_call(state, call),
            ast::Expr::Ident(ident) => self.eval_symbol(state, ident),
            ast::Expr::Literal(literal, _) => Some(Value::from(literal.to_owned())),
            ast::Expr::Content(content) => self.eval_content(state, content),
        }
    }

    fn eval_content(&self, state: &mut Self::State, content: &ast::Content) -> Option<Value<S>> {
        let mut result = Vec::new();

        for inline in &content.content {
            let evaluated = self.eval_inline(state, inline)?;
            result.push(Value::Inline(evaluated));
        }

        Some(Value::List(result))
    }

    fn eval_symbol(&self, state: &mut Self::State, ident: &ast::Ident) -> Option<Value<S>> {
        let key = &ident.ident;

        let value = match state.symbol(key) {
            Some(v) => v,
            None => {
                state.error(EngineError::new(
                    ident.span,
                    SymbolNotFound(key.to_string()),
                ));
                return None;
            }
        };

        Some(value)
    }

    fn eval_call(&self, state: &mut Self::State, call: &ast::Call) -> Option<Value<S>> {
        let ast::Call { ident, args, span } = call;

        let key = &ident.ident;
        let cmd = match state.command(key) {
            Some(cmd) => cmd,
            None => {
                state.error(EngineError::new(*span, FunctionNotFound(key.to_string())));
                return None;
            }
        };

        let args = self.eval_args(state, args)?;
        let call = UnverifiedCall { args, span: *span };
        match cmd.dispatch(call, state) {
            Ok(val) => Some(val),
            Err(errs) => {
                state.errors(errs);
                None
            }
        }
    }

    fn eval_args(&self, state: &mut Self::State, args: &ast::Args) -> Option<Vec<Arg<S>>> {
        let ast::Args {
            args,
            content,
            span: _,
        } = args;

        let mut args = args
            .iter()
            .map(|arg| self.eval_arg(state, arg))
            .collect::<Option<Vec<_>>>()?;

        if let Some(content) = content {
            let span = content.span;
            let value = self.eval_content(state, content)?;

            args.push(Arg {
                name: None,
                span,
                value,
            });
        }

        Some(args)
    }

    fn eval_arg(&self, state: &mut Self::State, arg: &ast::Arg) -> Option<Arg<S>> {
        let ast::Arg { name, value, span } = arg;

        let value = self.eval_expr(state, value)?;

        Some(Arg {
            name: name.as_ref().map(|s| s.to_string()),
            span: *span,
            value,
        })
    }
}
