use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tyd_pandoc::builtin;
use tyd_pandoc::engine::{PandocEngine, PandocState};
use tyd_render::context::Context;
use tyd_syntax::ast::Ast;
use tyd_syntax::parser::try_parse;
use tyd_syntax::visitor::Visitor;

use crate::tree::SyntaxNode;

#[derive(Debug)]
pub struct Backend {
    client: Client,
    documents: DashMap<Url, Rope>,
    abstract_trees: DashMap<Url, Ast>,
    trees: DashMap<Url, SyntaxNode>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DashMap::new(),
            abstract_trees: DashMap::new(),
            trees: DashMap::new(),
        }
    }

    pub async fn on_change(&self, uri: Url, source: String, version: i32) {
        let rope = Rope::from_str(&source);
        let result = try_parse(&source);

        let mut diags = result
            .errors()
            .filter_map(|e| {
                Some(Diagnostic {
                    range: range_conversion(e.span().into_range(), &rope)?,
                    message: e.to_string(),
                    ..Default::default()
                })
            })
            .collect::<Vec<_>>();

        if let Some(ast) = result.into_output() {
            // TODO do pandoc compilation here and check results
            // TODO make lsp generic over compiler

            let path = uri.to_file_path().unwrap();
            let name = path.file_name().unwrap().to_string_lossy().to_string();

            let mut state = PandocState::new(&source, name, path)
                .register("hrule", builtin::HorizontalRule)
                .register("figure", builtin::Figure)
                // Inlines
                .register("image", builtin::Image)
                .register("linebreak", builtin::LineBreak)
                .register("highlight", builtin::Highlight)
                .register("smallcaps", builtin::SmallCaps)
                .register("underline", builtin::Underline)
                // Builtins
                .register("let", builtin::Let)
                .register("List", builtin::List)
                .register("Map", builtin::Map);

            let engine = PandocEngine::new();

            // TODO dont want to panic if visit_ast has unrecoverable error
            // maybe rewrite visitor to not return errors
            engine.visit_ast(&mut state, &ast);

            let mut engine_diags = state
                .into_errors()
                .into_iter()
                .filter_map(|e| {
                    Some(Diagnostic {
                        range: range_conversion(e.span.into_range(), &rope)?,
                        message: e.msg.to_string(),
                        ..Default::default()
                    })
                })
                .collect();
            diags.append(&mut engine_diags);

            // then do conversion into cst and create semantic tokens

            self.abstract_trees.insert(uri.clone(), ast);
        }

        self.client
            .publish_diagnostics(uri.clone(), diags, Some(version))
            .await;

        self.documents.insert(uri, rope);
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;
        self.on_change(
            params.text_document.uri,
            params.text_document.text,
            params.text_document.version,
        )
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file changed!")
            .await;
        self.on_change(
            params.text_document.uri,
            std::mem::take(&mut params.content_changes[0].text),
            params.text_document.version,
        )
        .await
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }
    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;
    }
}

fn range_conversion(span: std::ops::Range<usize>, rope: &Rope) -> Option<Range> {
    let start = offset_to_position(span.start, rope)?;
    let end = offset_to_position(span.end, rope)?;
    Some(Range::new(start, end))
}

fn offset_to_position(offset: usize, rope: &Rope) -> Option<Position> {
    let line = rope.try_char_to_line(offset).ok()?;
    let first_char_of_line = rope.try_line_to_char(line).ok()?;
    let column = offset - first_char_of_line;
    Some(Position::new(line as u32, column as u32))
}
