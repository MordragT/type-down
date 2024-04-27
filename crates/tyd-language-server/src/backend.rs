use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tyd_eval::eval::{Context, Scope};
use tyd_eval::value::Value;
use tyd_eval::world::World;
use tyd_pandoc::builtin;
use tyd_pandoc::engine::PandocEngine;
use tyd_syntax::ast::Ast;
use tyd_syntax::parser::try_parse;
use tyd_syntax::visitor::Visitor;

use crate::semantic::{semantic_tokens_full_from_node, semantic_tokens_range_from_node, LEGEND};
use crate::syntax::SyntaxNode;

#[derive(Debug)]
pub struct Backend {
    client: Client,
    documents: DashMap<Url, Rope>,
    trees: DashMap<Url, Ast>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DashMap::new(),
            trees: DashMap::new(),
        }
    }

    pub async fn on_semantic_tokens_full(&self, uri: Url) -> Option<SemanticTokensResult> {
        let rope = self.documents.get(&uri)?;
        let ast = self.trees.get(&uri)?;
        let node = SyntaxNode::from(ast.value());
        let semantic_tokens = semantic_tokens_full_from_node(node, &rope);

        self.client
            .log_message(
                MessageType::INFO,
                format!("semantic tokens full: {}", semantic_tokens.len()),
            )
            .await;

        Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: semantic_tokens,
        }))
    }

    pub async fn on_semantic_tokens_range(&self, uri: Url) -> Option<SemanticTokensRangeResult> {
        let rope = self.documents.get(&uri)?;
        let ast = self.trees.get(&uri)?;
        let node = SyntaxNode::from(ast.value());
        // let semantic_tokens = semantic_tokens_range_from_node(node, &rope);
        let semantic_tokens = semantic_tokens_full_from_node(node, &rope);

        Some(SemanticTokensRangeResult::Tokens(SemanticTokens {
            result_id: None,
            data: semantic_tokens,
        }))
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
            let scope = Scope::new()
                .register_symbol("title", "Default title")
                .register_symbol("author", vec![Value::from("Max Mustermann")])
                //Blocks
                .register_func("hrule", builtin::HorizontalRule)
                .register_func("figure", builtin::Figure)
                // Inlines
                .register_func("image", builtin::Image)
                .register_func("linebreak", builtin::LineBreak)
                .register_func("highlight", builtin::Highlight)
                .register_func("smallcaps", builtin::SmallCaps)
                .register_func("underline", builtin::Underline)
                // Builtins
                .register_func("let", builtin::Let)
                .register_func("List", builtin::List)
                .register_func("Map", builtin::Map);

            let world = World::new(path, scope).unwrap();
            let mut engine = PandocEngine::new(world.clone());

            // TODO dont want to panic if visit_ast has unrecoverable error
            // maybe rewrite visitor to not return errors
            let mut context = Context::new(world);
            engine.visit_ast(&mut context, &ast);

            let mut engine_diags = context
                .scope
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
            self.trees.insert(uri.clone(), ast);
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
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                        SemanticTokensRegistrationOptions {
                            text_document_registration_options: {
                                TextDocumentRegistrationOptions {
                                    document_selector: Some(vec![DocumentFilter {
                                        language: Some("tyd".to_owned()),
                                        scheme: Some("file".to_owned()),
                                        pattern: None,
                                    }]),
                                }
                            },
                            semantic_tokens_options: SemanticTokensOptions {
                                work_done_progress_options: WorkDoneProgressOptions::default(),
                                legend: SemanticTokensLegend {
                                    token_types: LEGEND.into(),
                                    token_modifiers: vec![],
                                },
                                range: Some(true),
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                            },
                            static_registration_options: StaticRegistrationOptions::default(),
                        },
                    ),
                ),
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

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        Ok(self.on_semantic_tokens_full(uri).await)
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        let uri = params.text_document.uri;
        Ok(self.on_semantic_tokens_range(uri).await)
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
