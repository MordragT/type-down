use tyd_syntax::ast;

use crate::{
    error::{EngineError, EngineMessage},
    hir,
    value::Value,
};

mod engine;
mod scope;
mod tracer;

pub use engine::*;
pub use scope::*;
pub use tracer::*;

/// Evaluate an expression.
pub trait Eval<E: Engine> {
    type Output;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output;
}

impl<E: Engine> Eval<E> for &ast::Expr {
    type Output = Option<Value<E>>;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output {
        match self {
            ast::Expr::Block(block, _) => todo!(),
            ast::Expr::Call(call) => call.eval(engine, visitor),
            ast::Expr::Ident(ident) => ident.eval(engine, visitor),
            ast::Expr::Literal(literal, _) => Some(literal.eval(engine, visitor)),
            ast::Expr::Content(content) => content.eval(engine, visitor),
        }
    }
}

impl<E: Engine> Eval<E> for &ast::Content {
    type Output = Option<Value<E>>;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output {
        let mut result = Vec::new();

        for inline in &self.content {
            let evaluated = engine.eval_inline(visitor, inline)?;
            result.push(Value::Inline(evaluated));
        }

        Some(Value::List(result.into()))
    }
}

impl<E: Engine> Eval<E> for &ast::Ident {
    type Output = Option<Value<E>>;

    fn eval(self, engine: &mut E, _visitor: &E::Visitor) -> Self::Output {
        let key = &self.ident;

        match engine.scopes().symbol(key) {
            Some(v) => Some(v),
            None => {
                engine.tracer_mut().error(EngineError::new(
                    self.span,
                    EngineMessage::SymbolNotFound(key.clone()),
                ));
                None
            }
        }
    }
}

impl<E: Engine> Eval<E> for hir::Call<E> {
    type Output = Option<Value<E>>;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output {
        let hir::Call { ident, args, span } = self;

        match engine.scopes().func(&ident) {
            Some(f) => f.call(args, engine, visitor),
            None => {
                engine.tracer_mut().error(EngineError::new(
                    span,
                    EngineMessage::FunctionNotFound(ident),
                ));
                None
            }
        }
    }
}

impl<E: Engine> Eval<E> for &ast::Call {
    type Output = Option<Value<E>>;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output {
        let ast::Call { ident, args, span } = self;
        let args = args.eval(engine, visitor)?;
        let call = hir::Call {
            ident: ident.ident.clone(),
            args,
            span: *span,
        };
        call.eval(engine, visitor)
    }
}

impl<E: Engine> Eval<E> for &ast::Args {
    type Output = Option<hir::Args<E>>;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output {
        let ast::Args {
            args,
            content,
            span,
        } = self;

        let mut result = hir::Args::new(*span);

        for arg in args {
            let arg = arg.eval(engine, visitor)?;
            result.insert(arg);
        }

        if let Some(content) = content {
            let span = content.span;
            let value = content.eval(engine, visitor)?;

            result.insert(hir::Arg {
                name: None,
                span,
                value,
            });
        }

        Some(result)
    }
}

impl<E: Engine> Eval<E> for &ast::Arg {
    type Output = Option<hir::Arg<E>>;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output {
        let ast::Arg { name, value, span } = self;

        let value = value.eval(engine, visitor)?;

        Some(hir::Arg {
            name: name.as_ref().map(|ident| ident.ident.clone()),
            span: *span,
            value,
        })
    }
}

impl<E: Engine> Eval<E> for &ast::Literal {
    type Output = Value<E>;

    fn eval(self, _engine: &mut E, _visitor: &E::Visitor) -> Self::Output {
        use ast::Literal::*;

        match self {
            Boolean(b) => Value::Bool(*b),
            Str(s) => Value::Str(s.clone()),
            Int(i) => Value::Int(*i),
        }
    }
}
