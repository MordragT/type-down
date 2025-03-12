use tyd_doc::{id::NodeId, tree};

use crate::{
    error::{EngineError, EngineMessage},
    ir, ty,
    value::Value,
};

mod engine;
mod scope;
mod tracer;

pub use engine::*;
pub use scope::*;
pub use tracer::*;

/// Evaluate an expression.
pub trait Eval {
    type Output;

    fn eval(self, engine: &mut Engine) -> Self::Output;
}

impl<T: Eval> Eval for NodeId<T> {
    type Output = T::Output;

    fn eval(self, engine: &mut Engine) -> Self::Output {
        todo!()
    }
}

impl Eval for tree::Expr {
    type Output = Option<Value>;

    fn eval(self, engine: &mut Engine) -> Self::Output {
        match self {
            tree::Expr::Block(block) => todo!(),
            tree::Expr::Call(call) => call.eval(engine),
            tree::Expr::Ident(ident) => ident.eval(engine),
            tree::Expr::Literal(literal) => Some(literal.eval(engine)),
            tree::Expr::Content(content) => content.eval(engine),
        }
    }
}

impl Eval for tree::Content {
    type Output = Option<Value>;

    fn eval(self, engine: &mut Engine) -> Self::Output {
        let mut result = Vec::new();

        for inline in self {
            let evaluated = engine.eval_inline(inline)?;
            result.push(Value::Inline(evaluated));
        }

        Some(Value::List(result.into()))
    }
}

impl Eval for tree::Ident {
    type Output = Option<Value>;

    fn eval(self, engine: &mut Engine) -> Self::Output {
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

impl Eval for ir::Call {
    type Output = Option<Value>;

    fn eval(self, engine: &mut Engine) -> Self::Output {
        let ir::Call { ident, args, span } = self;

        match engine.scopes().func(&ident) {
            Some(f) => f.call(args, engine),
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

impl Eval for tree::Call {
    type Output = Option<Value>;

    fn eval(self, engine: &mut Engine) -> Self::Output {
        let ident = self.ident().get();
        let args = self.args();

        let args = args.eval(engine)?;
        let call = hir::Call {
            ident: ident.clone(),
            args,
            span: self.span(),
        };
        call.eval(engine)
    }
}

impl Eval for tree::Args {
    type Output = Option<ir::Args>;

    fn eval(self, engine: &mut Engine) -> Self::Output {
        let mut result = ir::Args::new(self.span());

        for arg in self {
            let arg = arg.eval(engine)?;
            result.insert(arg);
        }

        if let Some(content) = self.content() {
            let span = content.span();

            engine.scopes_mut().enter();
            let value = content.eval(engine)?;
            engine.scopes_mut().exit();

            result.insert(ir::Arg {
                name: None,
                span,
                value,
            });
        }

        Some(result)
    }
}

impl Eval for tree::Arg {
    type Output = Option<ir::Arg>;

    fn eval(self, engine: &mut Engine) -> Self::Output {
        let name = self.ident();
        let value = self.value();

        let value = value.eval(engine)?;

        Some(ir::Arg {
            name: name.as_ref().map(|ident| ident.get().clone()),
            span: self.span(),
            value,
        })
    }
}

impl Eval for tree::Literal {
    type Output = Value;

    fn eval(self, _engine: &mut Engine) -> Self::Output {
        use tree::Literal::*;

        match self {
            Bool(b) => Value::Bool(b),
            Str(s) => Value::Str(s),
            Int(i) => Value::Int(i),
            Float(f) => Value::Float(f),
        }
    }
}
