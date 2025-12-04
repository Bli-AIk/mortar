use ropey::Rope;
use tower_lsp_server::LanguageServer;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::lsp_types::*;
use tracing::info;

use crate::backend::Backend;

impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let mut language = self.get_language().await; // 使用之前设置的语言或默认语言

        // Check for language preference in client info or initialization options
        if let Some(client_info) = &params.client_info {
            info!(
                "{}: {}",
                crate::backend::i18n::get_lsp_text("client_connected", language),
                client_info.name
            );
        }

        // Try to detect language from initialization options (overrides command line)
        if let Some(init_options) = &params.initialization_options
            && let Ok(options) = serde_json::from_value::<serde_json::Map<String, serde_json::Value>>(
                init_options.clone(),
            )
            && let Some(lang_value) = options.get("language")
            && let Ok(lang_str) = serde_json::from_value::<String>(lang_value.clone())
        {
            language = match lang_str.as_str() {
                "zh" | "chinese" => mortar_compiler::Language::Chinese,
                _ => mortar_compiler::Language::English,
            };
            self.set_language(language).await;
            info!(
                "{}: {:?}",
                crate::backend::i18n::get_lsp_text("language_set_to", language),
                language
            );
        }

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
                                    SemanticTokenType::METHOD, // Used for function calls
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
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["mortar.setLanguage".to_string()],
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                }),
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
        let language = self.get_language().await;
        info!(
            "{}",
            crate::backend::i18n::get_lsp_text("shutdown_requested", language)
        );

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

        // Use the new analyze_document method
        self.analyze_document(&uri, &content).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = Some(params.text_document.version);

        if let Some(change) = params.content_changes.into_iter().next() {
            let content = change.text;
            let rope = Rope::from_str(&content);
            self.documents.insert(uri.clone(), (rope, version));

            // Cancel any existing debounce task for this URI
            if let Some(handle) = self.debouncers.get(&uri) {
                handle.abort();
            }

            let backend = self.clone();
            let uri_clone = uri.clone();
            let content_clone = content.clone();

            let task = tokio::spawn(async move {
                // Debounce delay
                tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
                backend.analyze_document(&uri_clone, &content_clone).await;
            });

            self.debouncers.insert(uri, task);
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;

        // Cancel and remove any pending debounce task
        if let Some(handle) = self.debouncers.remove(&uri) {
            handle.1.abort();
        }

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

    async fn execute_command(
        &self,
        params: ExecuteCommandParams,
    ) -> Result<Option<serde_json::Value>> {
        match params.command.as_str() {
            "mortar.setLanguage" => {
                if !params.arguments.is_empty()
                    && let Some(lang_arg) = params.arguments.first()
                    && let Ok(lang_str) = serde_json::from_value::<String>(lang_arg.clone())
                {
                    let language = match lang_str.as_str() {
                        "en" | "english" => mortar_compiler::Language::English,
                        "zh" | "chinese" => mortar_compiler::Language::Chinese,
                        _ => {
                            let error_msg = crate::backend::i18n::get_lsp_text(
                                "unsupported_language_error",
                                self.get_language().await,
                            );
                            return Ok(Some(serde_json::json!({
                                "error": error_msg
                            })));
                        }
                    };

                    self.set_language(language).await;

                    // Re-analyze all open documents with the new language
                    let documents_snapshot: Vec<(Uri, String)> = self
                        .documents
                        .iter()
                        .map(|entry| {
                            let uri = entry.key().clone();
                            let content = entry.value().0.to_string();
                            (uri, content)
                        })
                        .collect();

                    for (uri, content) in documents_snapshot {
                        self.analyze_document(&uri, &content).await;
                    }

                    let success_msg =
                        crate::backend::i18n::get_lsp_text("language_changed_success", language);
                    return Ok(Some(serde_json::json!({
                        "message": success_msg,
                        "language": lang_str
                    })));
                }
                let error_msg = crate::backend::i18n::get_lsp_text(
                    "invalid_command_arguments",
                    self.get_language().await,
                );
                Ok(Some(serde_json::json!({
                    "error": error_msg
                })))
            }
            _ => Ok(None),
        }
    }
}
