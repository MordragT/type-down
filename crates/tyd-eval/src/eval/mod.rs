use tyd_syntax::ast;

use crate::{
    error::{EngineError, EngineMessage},
    foundations::{Arg, Call},
    value::Value,
};

mod context;
mod engine;
mod machine;
mod scope;

pub use context::*;
pub use engine::*;
pub use machine::*;
pub use scope::*;

/// Evaluate an expression.
pub trait Eval<E: Engine> {
    type Output: Into<Value<E>>;

    fn eval(&self, machine: &mut Machine<E>) -> Self::Output;
}

impl<E: Engine> Eval<E> for ast::Expr {
    type Output = Option<Value<E>>;

    fn eval(&self, machine: &mut Machine<E>) -> Self::Output {
        match self {
            ast::Expr::Block(block, _) => todo!(),
            ast::Expr::Call(call) => call.eval(machine),
            ast::Expr::Ident(ident) => ident.eval(machine),
            ast::Expr::Literal(literal, _) => Some(literal.eval(machine)),
            ast::Expr::Content(content) => content.eval(machine),
        }
    }
}

impl<E: Engine> Eval<E> for ast::Content {
    type Output = Option<Value<E>>;

    fn eval(&self, machine: &mut Machine<E>) -> Self::Output {
        let mut result = Vec::new();

        for inline in &self.content {
            let evaluated = machine.engine.process_inline(inline)?;
            result.push(Value::Inline(evaluated));
        }

        Some(Value::List(result.into()))
    }
}

impl<E: Engine> Eval<E> for ast::Ident {
    type Output = Option<Value<E>>;

    fn eval(&self, machine: &mut Machine<E>) -> Self::Output {
        let key = &self.ident;

        let value = match machine.symbol(key) {
            Some(v) => v,
            None => {
                machine.scope.error(EngineError::new(
                    self.span,
                    EngineMessage::SymbolNotFound(key.clone()),
                ));
                return None;
            }
        };

        Some(value)
    }
}

impl<E: Engine> Eval<E> for ast::Call {
    type Output = Option<Value<E>>;

    fn eval(&self, machine: &mut Machine<E>) -> Self::Output {
        let ast::Call { ident, args, span } = self;

        let key = &ident.ident;
        let f = match machine.func(key) {
            Some(cmd) => cmd,
            None => {
                machine.scope.error(EngineError::new(
                    *span,
                    EngineMessage::FunctionNotFound(key.clone()),
                ));
                return None;
            }
        };

        let args = args.eval(machine)?;
        f.dispatch(Call { args, span: *span }, machine)
    }
}

impl<E: Engine> Eval<E> for ast::Args {
    type Output = Option<Vec<Arg<E>>>;

    fn eval(&self, machine: &mut Machine<E>) -> Self::Output {
        let ast::Args {
            args,
            content,
            span: _,
        } = self;

        let mut args = args
            .iter()
            .map(|arg| arg.eval(machine))
            .collect::<Option<Vec<_>>>()?;

        if let Some(content) = content {
            let span = content.span;
            let value = content.eval(machine)?;

            args.push(Arg {
                name: None,
                span,
                value,
            });
        }

        Some(args)
    }
}

impl<E: Engine> Eval<E> for ast::Arg {
    type Output = Option<Arg<E>>;

    fn eval(&self, machine: &mut Machine<E>) -> Self::Output {
        let ast::Arg { name, value, span } = self;

        let value = value.eval(machine)?;

        Some(Arg {
            name: name.as_ref().map(|n| n.ident.clone()),
            span: *span,
            value,
        })
    }
}

impl<E: Engine> Eval<E> for ast::Literal {
    type Output = Value<E>;

    fn eval(&self, _machine: &mut Machine<E>) -> Self::Output {
        use ast::Literal::*;

        match self {
            Boolean(b) => Value::Bool(*b),
            Str(s) => Value::Str(s.clone()),
            Int(i) => Value::Int(*i),
        }
    }
}
