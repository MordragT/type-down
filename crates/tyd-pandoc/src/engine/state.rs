use miette::NamedSource;
use pandoc_ast as ir;
use std::{collections::BTreeMap, sync::Arc};
use tyd_render::{error::EngineErrorHandler, SymbolTable};

use crate::{CommandBox, PandocShape, Value};

pub struct PandocState {
    pub(crate) pandoc: ir::Pandoc,
    stack: Vec<ir::Inline>,
    symbols: BTreeMap<String, Value>,
    commands: BTreeMap<String, CommandBox>,
    named_source: NamedSource<Arc<str>>,
}

impl PandocState {
    pub fn new(source: impl AsRef<str>, name: impl AsRef<str>) -> Self {
        Self {
            pandoc: ir::Pandoc {
                pandoc_api_version: vec![1, 23, 1],
                meta: BTreeMap::new(),
                blocks: Vec::new(),
            },
            stack: Vec::new(),
            symbols: BTreeMap::new(),
            commands: BTreeMap::new(),
            named_source: NamedSource::new(name, Arc::from(source.as_ref())),
        }
    }

    pub fn register(
        mut self,
        name: impl Into<String>,
        command: impl tyd_render::Command<PandocShape> + 'static,
    ) -> Self {
        self.commands.insert(name.into(), Arc::new(command));
        self
    }

    pub fn insert(mut self, name: impl Into<String>, value: impl Into<Value>) -> Self {
        self.symbols.insert(name.into(), value.into());
        self
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

impl SymbolTable<PandocShape> for PandocState {
    fn command(&self, key: impl AsRef<str>) -> Option<CommandBox> {
        self.commands.get(key.as_ref()).cloned()
    }

    fn symbol(&self, key: impl AsRef<str>) -> Option<Value> {
        self.symbols.get(key.as_ref()).cloned()
    }
}

impl EngineErrorHandler for PandocState {
    fn named_source(&self) -> NamedSource<Arc<str>> {
        self.named_source.clone()
    }
}
