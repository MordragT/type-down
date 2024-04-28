use miette::Result;
use pandoc_ast as ir;
use tyd_eval::prelude::*;
use tyd_syntax::{ast, visitor::Visitor};

use crate::{error::PandocError, visitor::PandocVisitor};

impl Cast<PandocEngine> for ir::Inline {
    fn cast(value: Value<PandocEngine>) -> Self {
        value.into_inline().unwrap()
    }
}

impl Cast<PandocEngine> for ir::Block {
    fn cast(value: Value<PandocEngine>) -> Self {
        value.into_block().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct PandocEngine {
    pandoc: ir::Pandoc,
    stack: Vec<ir::Inline>,
    world: World<Self>,
    scopes: Scopes<Self>,
    tracer: Tracer,
}

impl PandocEngine {
    pub fn new(world: World<Self>) -> Self {
        Self {
            pandoc: ir::Pandoc {
                pandoc_api_version: vec![1, 23, 1],
                meta: ir::Map::new(),
                blocks: Vec::new(),
            },
            stack: Vec::new(),
            scopes: Scopes::new(world.global_scope()),
            tracer: Tracer::new(),
            world,
        }
    }

    pub fn build(mut self, ast: &ast::Ast) -> Result<ir::Pandoc, PandocError> {
        PandocVisitor {}.visit_ast(&mut self, ast)?;

        let Self {
            mut pandoc,
            stack,
            world,
            scopes,
            tracer,
        } = self;

        assert!(stack.is_empty());

        if tracer.has_errors() {
            return Err(EngineErrors {
                src: world.named_source(),
                related: tracer.into_errors(),
            })?;
        }

        if let Some(Value::Str(title)) = scopes.symbol("title") {
            pandoc.meta.insert(
                "title".to_owned(),
                ir::MetaValue::MetaString(title.to_string()),
            );
        }

        Ok(pandoc)
    }

    pub(crate) fn start(&self) -> usize {
        self.stack.len()
    }

    pub(crate) fn end(&mut self, start: usize) -> impl Iterator<Item = ir::Inline> + '_ {
        self.stack.drain(start..)
    }

    pub(crate) fn take_stack(&mut self) -> Vec<ir::Inline> {
        std::mem::replace(&mut self.stack, Vec::new())
    }

    pub(crate) fn stack_is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub(crate) fn push(&mut self, inline: ir::Inline) {
        self.stack.push(inline);
    }

    pub(crate) fn add_block(&mut self, block: ir::Block) {
        self.pandoc.blocks.push(block)
    }

    pub(crate) fn pop_block(&mut self) -> ir::Block {
        self.pandoc.blocks.pop().unwrap()
    }
}

impl Engine for PandocEngine {
    type Inline = ir::Inline;
    type Block = ir::Block;
    type Visitor = PandocVisitor;

    fn eval_block(&mut self, visitor: &Self::Visitor, block: &ast::Block) -> Option<Self::Block> {
        visitor.visit_block(self, block).ok()?;
        let block = self.pop_block();
        Some(block)
    }

    fn eval_inline(
        &mut self,
        visitor: &Self::Visitor,
        inline: &ast::Inline,
    ) -> Option<Self::Inline> {
        let start = self.start();
        visitor.visit_inline(self, inline).ok()?;

        // TODO error handling

        let content = self.end(start).last().unwrap();

        Some(content)
    }

    fn world(&self) -> World<Self> {
        self.world.clone()
    }

    fn scopes(&self) -> &Scopes<Self> {
        &self.scopes
    }

    fn scopes_mut(&mut self) -> &mut Scopes<Self> {
        &mut self.scopes
    }

    fn tracer(&self) -> &Tracer {
        &self.tracer
    }

    fn tracer_mut(&mut self) -> &mut Tracer {
        &mut self.tracer
    }
}
