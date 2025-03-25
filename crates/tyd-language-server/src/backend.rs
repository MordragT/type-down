use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer};
use tyd_core::prelude::*;
use tyd_eval::prelude::*;
use tyd_syntax::prelude::*;

use crate::semantic::{SemanticAnalyzer, LEGEND};

#[derive(Debug)]
pub struct Backend {
    client: Client,
    sources: DashMap<Url, Source>,
    documents: DashMap<Url, Doc>,
    spans: DashMap<Url, Spans>,
    global_scope: Scope,
}

impl Backend {
    pub fn new(client: Client, global_scope: Scope) -> Self {
        Self {
            client,
            sources: DashMap::new(),
            documents: DashMap::new(),
            spans: DashMap::new(),
            global_scope: global_scope.into(),
        }
    }

    pub async fn on_semantic_tokens_full(&self, uri: Url) -> Option<SemanticTokensResult> {
        let source = self.sources.get(&uri)?;
        let spans = self.spans.get(&uri)?;
        let doc = self.documents.get(&uri)?;

        let analyzer = SemanticAnalyzer::new(source.as_rope(), doc.clone(), spans.clone());

        let semantic_tokens = analyzer.tokens();

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
        let source = self.sources.get(&uri)?;
        let spans = self.spans.get(&uri)?;
        let doc = self.documents.get(&uri)?;

        let analyzer = SemanticAnalyzer::new(source.as_rope(), doc.clone(), spans.clone());

        let semantic_tokens = analyzer.tokens();

        Some(SemanticTokensRangeResult::Tokens(SemanticTokens {
            result_id: None,
            data: semantic_tokens,
        }))
    }

    pub async fn on_change(&self, uri: Url, source: String, version: i32) {
        let path = uri.to_file_path().unwrap();
        let name = path.file_name().unwrap().to_str().unwrap();

        let source = Source::new(&path, name, source);

        self.sources.insert(uri.clone(), source.clone());

        let ParseResult { doc, spans, errors } = parse(&source);

        self.spans.insert(uri.clone(), spans.clone());

        let mut tracer = Tracer::with_diagnostics(errors, source.clone(), spans.clone());

        if let Some(doc) = doc {
            self.documents.insert(uri.clone(), doc.clone());

            tracer = Engine::new(self.global_scope.clone(), tracer)
                .run(doc)
                .tracer;
        }

        let diags = tracer_into_diagnostic(tracer, &source.as_rope());

        self.client
            .publish_diagnostics(uri, diags, Some(version))
            .await;
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

fn tracer_into_diagnostic(tracer: Tracer, rope: &Rope) -> Vec<Diagnostic> {
    tracer
        .into_inner()
        .0
        .into_iter()
        .filter_map(|message| {
            Some(Diagnostic {
                range: range_conversion(message.span.into_range(), &rope)?,
                severity: match message.severity {
                    miette::Severity::Error => Some(DiagnosticSeverity::ERROR),
                    miette::Severity::Warning => Some(DiagnosticSeverity::WARNING),
                    miette::Severity::Advice => Some(DiagnosticSeverity::INFORMATION),
                },
                message: message.to_string(),
                ..Default::default()
            })
        })
        .collect()
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
