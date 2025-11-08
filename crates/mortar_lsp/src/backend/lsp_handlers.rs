use ropey::Rope;
use tokio;
use tower_lsp_server::LanguageServer;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::lsp_types::*;
use tracing::info;

use crate::analysis::SymbolTable;
use crate::backend::Backend;

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![".".to_string()]),
                    all_commit_characters: None,
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                    completion_item: None,
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                    retrigger_characters: None,
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                }),
                definition_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            work_done_progress_options: WorkDoneProgressOptions::default(),
                            legend: SemanticTokensLegend {
                                token_types: vec![
                                    SemanticTokenType::KEYWORD,
                                    SemanticTokenType::STRING,
                                    SemanticTokenType::NUMBER,
                                    SemanticTokenType::COMMENT,
                                    SemanticTokenType::FUNCTION,
                                    SemanticTokenType::VARIABLE,
                                    SemanticTokenType::TYPE,
                                    SemanticTokenType::OPERATOR,
                                    SemanticTokenType::new("punctuation"),
                                ],
                                token_modifiers: vec![
                                    SemanticTokenModifier::DECLARATION,
                                    SemanticTokenModifier::DEFINITION,
                                    SemanticTokenModifier::READONLY,
                                ],
                            },
                            range: Some(true),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                        },
                    ),
                ),
                ..ServerCapabilities::default()
            },
            ..InitializeResult::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Mortar LSP server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        info!("Close request received");

        self.documents.clear();
        self.diagnostics.clear();
        self.symbol_tables.clear();

        info!("Shutdown completed");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;
        let version = Some(params.text_document.version);

        let rope = Rope::from_str(&content);
        self.documents.insert(uri.clone(), (rope, version));

        let client = self.client.clone();
        let symbol_tables = self.symbol_tables.clone();
        let diagnostics = self.diagnostics.clone();
        let uri_clone = uri.clone();
        let content_clone = content.clone();

        tokio::spawn(async move {
            let mut diagnostics_vec = Vec::new();
            let symbol_table = SymbolTable::new();

            if content_clone.trim().is_empty() {
                diagnostics_vec.push(Diagnostic::new_simple(
                    Range::new(Position::new(0, 0), Position::new(0, 0)),
                    "Document is empty".to_string(),
                ));
            } else if !content_clone.contains("node") {
                diagnostics_vec.push(Diagnostic::new_simple(
                    Range::new(Position::new(0, 0), Position::new(0, 0)),
                    "The document should contain at least one node definition".to_string(),
                ));
            }

            symbol_tables.insert(uri_clone.clone(), symbol_table);
            diagnostics.insert(uri_clone.clone(), diagnostics_vec.clone());

            let _ = client
                .publish_diagnostics(uri_clone, diagnostics_vec, None)
                .await;
        });
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = Some(params.text_document.version);

        if let Some(change) = params.content_changes.into_iter().next() {
            let content = change.text;
            let rope = Rope::from_str(&content);
            self.documents.insert(uri.clone(), (rope, version));

            let client = self.client.clone();
            let symbol_tables = self.symbol_tables.clone();
            let diagnostics = self.diagnostics.clone();
            let uri_clone = uri.clone();
            let content_clone = content.clone();

            tokio::spawn(async move {
                let mut diagnostics_vec = Vec::new();
                let symbol_table = SymbolTable::new();

                if content_clone.trim().is_empty() {
                    diagnostics_vec.push(Diagnostic::new_simple(
                        Range::new(Position::new(0, 0), Position::new(0, 0)),
                        "Document is empty".to_string(),
                    ));
                } else if !content_clone.contains("node") {
                    diagnostics_vec.push(Diagnostic::new_simple(
                        Range::new(Position::new(0, 0), Position::new(0, 0)),
                        "The document should contain at least one node definition".to_string(),
                    ));
                }

                symbol_tables.insert(uri_clone.clone(), symbol_table);
                diagnostics.insert(uri_clone.clone(), diagnostics_vec.clone());

                let _ = client
                    .publish_diagnostics(uri_clone, diagnostics_vec, None)
                    .await;
            });
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.remove(&uri);
        self.diagnostics.remove(&uri);
        self.symbol_tables.remove(&uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let (rope, _version) = match self.documents.get(uri) {
            Some(entry) => (entry.0.clone(), entry.1),
            None => return Ok(None),
        };

        let symbol_table = match self.symbol_tables.get(uri) {
            Some(table) => table.clone(),
            None => return Ok(None),
        };

        let line_idx = position.line as usize;
        let char_idx = position.character as usize;

        let line_content = if line_idx < rope.len_lines() {
            rope.line(line_idx).to_string()
        } else {
            String::new()
        };

        let current_word = self.get_current_word(&line_content, char_idx);
        let context = self.analyze_document_context(uri, position);

        // Use unified completion generation from completion.rs
        let completions =
            self.generate_completion_items_filtered(context, &current_word, &symbol_table);

        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let _uri = &params.text_document_position_params.text_document.uri;
        let _position = params.text_document_position_params.position;

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**Mortar Language**\n\nDSL for game dialogue and text event systems."
                    .to_string(),
            }),
            range: None,
        }))
    }

    async fn signature_help(&self, _params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        Ok(None)
    }

    async fn goto_definition(
        &self,
        _params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = &params.text_document.uri;

        let symbol_table = match self.symbol_tables.get(uri) {
            Some(table) => table.clone(),
            None => return Ok(None),
        };

        let mut symbols = Vec::new();

        for node in &symbol_table.nodes {
            symbols.push(DocumentSymbol {
                name: node.clone(),
                detail: None,
                kind: SymbolKind::CLASS,
                tags: None,
                #[allow(deprecated)]
                deprecated: Some(false),
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                selection_range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                children: None,
            });
        }

        for func in &symbol_table.functions {
            symbols.push(DocumentSymbol {
                name: func.name.clone(),
                detail: Some(format!(
                    "fn {}({}){}",
                    func.name,
                    func.params
                        .iter()
                        .map(|p| format!("{}: {}", p.name, p.type_name))
                        .collect::<Vec<_>>()
                        .join(", "),
                    func.return_type
                        .as_ref()
                        .map(|t| format!(" -> {}", t))
                        .unwrap_or_default()
                )),
                kind: SymbolKind::FUNCTION,
                tags: None,
                #[allow(deprecated)]
                deprecated: Some(false),
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                selection_range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                children: None,
            });
        }

        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = &params.text_document.uri;

        let (rope, _version) = match self.documents.get(uri) {
            Some(entry) => (entry.0.clone(), entry.1),
            None => return Ok(None),
        };

        let content = rope.to_string();
        let tokens = self.analyze_semantic_tokens(&content);

        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
        })))
    }
}
