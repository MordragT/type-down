use tyd_syntax::ast::{self, TypedNode};

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

impl<'a, E: Engine> Eval<E> for ast::Expr<'a> {
    type Output = Option<Value<E>>;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output {
        match self {
            ast::Expr::Block(block) => todo!(),
            ast::Expr::Call(call) => call.eval(engine, visitor),
            ast::Expr::Ident(ident) => ident.eval(engine, visitor),
            ast::Expr::Literal(literal) => Some(literal.eval(engine, visitor)),
            ast::Expr::Content(content) => content.eval(engine, visitor),
        }
    }
}

impl<'a, E: Engine> Eval<E> for ast::Content<'a> {
    type Output = Option<Value<E>>;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output {
        let mut result = Vec::new();

        for inline in self {
            let evaluated = engine.eval_inline(visitor, inline)?;
            result.push(Value::Inline(evaluated));
        }

        Some(Value::List(result.into()))
    }
}

impl<'a, E: Engine> Eval<E> for ast::Ident<'a> {
    type Output = Option<Value<E>>;

    fn eval(self, engine: &mut E, _visitor: &E::Visitor) -> Self::Output {
        let key = self.get();

        match engine.scopes().symbol(key) {
            Some(v) => Some(v),
            None => {
                engine.tracer_mut().error(EngineError::new(
                    self.span(),
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

impl<'a, E: Engine> Eval<E> for ast::Call<'a> {
    type Output = Option<Value<E>>;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output {
        let ident = self.ident().get();
        let args = self.args();

        let args = args.eval(engine, visitor)?;
        let call = hir::Call {
            ident: ident.clone(),
            args,
            span: self.span(),
        };
        call.eval(engine, visitor)
    }
}

impl<'a, E: Engine> Eval<E> for ast::Args<'a> {
    type Output = Option<hir::Args<E>>;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output {
        let mut result = hir::Args::new(self.span());

        for arg in self {
            let arg = arg.eval(engine, visitor)?;
            result.insert(arg);
        }

        if let Some(content) = self.content() {
            let span = content.span();

            engine.scopes_mut().enter();
            let value = content.eval(engine, visitor)?;
            engine.scopes_mut().exit();

            result.insert(hir::Arg {
                name: None,
                span,
                value,
            });
        }

        Some(result)
    }
}

impl<'a, E: Engine> Eval<E> for ast::Arg<'a> {
    type Output = Option<hir::Arg<E>>;

    fn eval(self, engine: &mut E, visitor: &E::Visitor) -> Self::Output {
        let name = self.ident();
        let value = self.value();

        let value = value.eval(engine, visitor)?;

        Some(hir::Arg {
            name: name.as_ref().map(|ident| ident.get().clone()),
            span: self.span(),
            value,
        })
    }
}

impl<'a, E: Engine> Eval<E> for ast::Literal<'a> {
    type Output = Value<E>;

    fn eval(self, _engine: &mut E, _visitor: &E::Visitor) -> Self::Output {
        use ast::Literal::*;

        match self {
            Bool(b) => Value::Bool(b.get()),
            Str(s) => Value::Str(s.get().clone()),
            Int(i) => Value::Int(i.get()),
            Float(f) => Value::Float(f.get()),
        }
    }
}
