use std::sync::Arc;

use dashmap::DashMap;
use mortar_compiler::ParseHandler;
use ropey::Rope;
use tokio::sync::RwLock;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::lsp_types::*;
use tower_lsp_server::{Client, LanguageServer};
use tracing::info;

use crate::analysis::{SymbolTable, analyze_program};
use crate::files::Files;

/// Autocomplete context type
#[derive(Debug, Clone, PartialEq)]
enum CompletionContext {
    // Top level: write node, fn, etc.
    TopLevel,
    // Inside the node: write text, events, choice, etc.
    InNode,
    // In the selection: write when, ->, node reference, etc.
    InChoice,
    //In expressions: function calls, variable references, etc.
    InExpression,
    Other,
}

#[derive(Clone)]
pub struct Backend {
    pub client: Client,
    pub files: Arc<RwLock<Files>>,
    pub documents: Arc<DashMap<Uri, (Rope, Option<i32>)>>, // (content, version)
    pub diagnostics: Arc<DashMap<Uri, Vec<Diagnostic>>>,
    pub symbol_tables: Arc<DashMap<Uri, SymbolTable>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            files: Arc::new(RwLock::new(Files::new())),
            documents: Arc::new(DashMap::new()),
            diagnostics: Arc::new(DashMap::new()),
            symbol_tables: Arc::new(DashMap::new()),
        }
    }

    /// Clear all cached data
    pub async fn cleanup(&self) {
        info!("Start cleaning up LSP server resources...");

        let documents_count = self.documents.len();
        self.documents.clear();
        info!("{} documents cleaned", documents_count);

        let diagnostics_count = self.diagnostics.len();
        self.diagnostics.clear();
        info!("Cleaned {} diagnostic messages", diagnostics_count);

        let symbols_count = self.symbol_tables.len();
        self.symbol_tables.clear();
        info!("Cleaned {} symbol tables", symbols_count);

        {
            let mut files = self.files.write().await;
            *files = Files::new();
        }

        info!("LSP server resource cleanup completed");
    }

    /// Simplified cleanup operations to avoid asynchronous blocking
    pub fn cleanup_sync(&self) {
        info!("Start synchronously cleaning up LSP server resources...");

        let documents_count = self.documents.len();
        self.documents.clear();

        let diagnostics_count = self.diagnostics.len();
        self.diagnostics.clear();

        let symbols_count = self.symbol_tables.len();
        self.symbol_tables.clear();

        info!(
            "Synchronization cleanup completed: {} documents, {} diagnostics, {} symbol tables",
            documents_count, diagnostics_count, symbols_count
        );
    }

    /// Analyze document content and generate diagnostic information
    async fn analyze_document(&self, uri: &Uri, content: &str) {
        let mut diagnostics = Vec::new();
        let mut symbol_table = SymbolTable::new();

        let content_owned = content.to_string();
        match tokio::task::spawn_blocking(move || ParseHandler::parse_source_code(&content_owned))
            .await
        {
            Ok(Ok(program)) => {
                match tokio::task::spawn_blocking(move || analyze_program(&program)).await {
                    Ok(Ok(table)) => {
                        symbol_table = table;
                    }
                    Ok(Err(errors)) => {
                        for (message, line) in errors {
                            diagnostics.push(Diagnostic::new_simple(
                                Range::new(
                                    Position::new(line.saturating_sub(1) as u32, 0),
                                    Position::new(line.saturating_sub(1) as u32, 0),
                                ),
                                message,
                            ));
                        }
                    }
                    Err(_) => {
                        diagnostics.push(Diagnostic::new_simple(
                            Range::new(Position::new(0, 0), Position::new(0, 0)),
                            "Analysis task failed".to_string(),
                        ));
                    }
                }
            }
            Ok(Err(error)) => {
                diagnostics.push(Diagnostic::new_simple(
                    Range::new(Position::new(0, 0), Position::new(0, 0)),
                    error,
                ));
            }
            Err(_) => {
                diagnostics.push(Diagnostic::new_simple(
                    Range::new(Position::new(0, 0), Position::new(0, 0)),
                    "Parse task failed".to_string(),
                ));
            }
        }

        self.symbol_tables.insert(uri.clone(), symbol_table);

        self.diagnostics.insert(uri.clone(), diagnostics.clone());

        let client = self.client.clone();
        let uri_clone = uri.clone();
        tokio::spawn(async move {
            let _ = client
                .publish_diagnostics(uri_clone, diagnostics, None)
                .await;
        });
    }

    /// Convert position to offset in document
    fn _position_to_offset(&self, rope: &Rope, position: Position) -> Option<usize> {
        let line_idx = position.line as usize;
        if line_idx >= rope.len_lines() {
            return None;
        }

        let line_start = rope.line_to_char(line_idx);
        let line_end = if line_idx + 1 < rope.len_lines() {
            rope.line_to_char(line_idx + 1)
        } else {
            rope.len_chars()
        };

        let char_idx = line_start + (position.character as usize);
        if char_idx <= line_end {
            Some(char_idx)
        } else {
            None
        }
    }
    /// Convert the offset to a position in the document
    fn _offset_to_position(&self, rope: &Rope, offset: usize) -> Option<Position> {
        if offset > rope.len_chars() {
            return None;
        }

        let line_idx = rope.char_to_line(offset);
        let line_start = rope.line_to_char(line_idx);
        let char_idx = offset - line_start;

        Some(Position::new(line_idx as u32, char_idx as u32))
    }
    /// Get the word at the current cursor position
    fn get_current_word(&self, line: &str, char_idx: usize) -> String {
        let chars: Vec<char> = line.chars().collect();
        let mut start = char_idx;
        let mut end = char_idx;

        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
            start -= 1;
        }

        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }

        chars[start..end].iter().collect()
    }
    /// Analyze the context of auto-completion
    fn analyze_completion_context(&self, line: &str, _char_idx: usize) -> CompletionContext {
        let trimmed = line.trim();

        if trimmed.is_empty() || (!trimmed.contains('{') && !trimmed.contains('}')) {
            return CompletionContext::TopLevel;
        }

        if trimmed.contains("choice") || trimmed.contains("->") {
            return CompletionContext::InChoice;
        }

        if trimmed.contains('(') || trimmed.contains('"') {
            return CompletionContext::InExpression;
        }

        if trimmed.contains("text") || trimmed.contains("events") {
            return CompletionContext::InNode;
        }

        CompletionContext::Other
    }
}

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
                                ],
                                token_modifiers: vec![],
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

        // 快速清理主要缓存，避免复杂异步操作
        self.documents.clear();
        self.diagnostics.clear();
        self.symbol_tables.clear();

        info!("关闭完成");
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

        let mut completions = Vec::new();

        let line_idx = position.line as usize;
        let char_idx = position.character as usize;

        let line_content = if line_idx < rope.len_lines() {
            rope.line(line_idx).to_string()
        } else {
            String::new()
        };

        let _text_before_cursor = if char_idx <= line_content.len() {
            &line_content[..char_idx]
        } else {
            &line_content
        };

        let current_word = self.get_current_word(&line_content, char_idx);

        let context = self.analyze_completion_context(&line_content, char_idx);

        match context {
            CompletionContext::TopLevel => {
                if "node".starts_with(&current_word) {
                    completions.push(CompletionItem {
                        label: "node".to_string(),
                        kind: Some(CompletionItemKind::KEYWORD),
                        insert_text: Some("node ${1:node_name} {\n\t$0\n}".to_string()),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        detail: Some("Create a dialogue node".to_string()),
                        documentation: Some(Documentation::String(
                            "Create a new dialog node that defines dialog content and choices"
                                .to_string(),
                        )),
                        ..CompletionItem::default()
                    });
                }
                if "nd".starts_with(&current_word) {
                    completions.push(CompletionItem {
                        label: "nd".to_string(),
                        kind: Some(CompletionItemKind::KEYWORD),
                        insert_text: Some("nd ${1:node_name} {\n\t$0\n}".to_string()),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        detail: Some("Create a dialogue node".to_string()),
                        documentation: Some(Documentation::String(
                            "Create a new dialog node that defines dialog content and choices"
                                .to_string(),
                        )),
                        ..CompletionItem::default()
                    });
                }
                if "fn".starts_with(&current_word) {
                    completions.push(CompletionItem {
                        label: "fn".to_string(),
                        kind: Some(CompletionItemKind::KEYWORD),
                        insert_text: Some(
                            "fn ${1:function_name}() {\n\t$0\n}"
                                .to_string(),
                        ),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        detail: Some("Create a function".to_string()),
                        documentation: Some(Documentation::String(
                            "Create a new function definition".to_string(),
                        )),
                        ..CompletionItem::default()
                    });
                }
            }
            CompletionContext::InNode => {
                for keyword in &["text", "events", "choice"] {
                    if keyword.starts_with(&current_word) {
                        let (insert_text, detail) = match *keyword {
                            "text" => (
                                "text: \"${1:content}\"".to_string(),
                                "Add conversation text".to_string(),
                            ),
                            "events" => (
                                "events: [\n\t${1:0}, ${2:event_function}()\n]".to_string(),
                                "Add event list".to_string(),
                            ),
                            "choice" => (
                                "choice: [\n\t\"${1:text}\" -> ${2:target_node}\n]".to_string(),
                                "Add selection".to_string(),
                            ),
                            _ => (keyword.to_string(), "".to_string()),
                        };
                        completions.push(CompletionItem {
                            label: keyword.to_string(),
                            kind: Some(CompletionItemKind::KEYWORD),
                            insert_text: Some(insert_text),
                            insert_text_format: Some(InsertTextFormat::SNIPPET),
                            detail: Some(detail),
                            ..CompletionItem::default()
                        });
                    }
                }
            }
            CompletionContext::InChoice => {
                for keyword in &["when", "return", "break"] {
                    if keyword.starts_with(&current_word) {
                        completions.push(CompletionItem {
                            label: keyword.to_string(),
                            kind: Some(CompletionItemKind::KEYWORD),
                            ..CompletionItem::default()
                        });
                    }
                }
                for node in &symbol_table.nodes {
                    if node.starts_with(&current_word) {
                        completions.push(CompletionItem {
                            label: node.clone(),
                            kind: Some(CompletionItemKind::CLASS),
                            detail: Some("Jump to node".to_string()),
                            ..CompletionItem::default()
                        });
                    }
                }
            }
            CompletionContext::InExpression => {
                for func in &symbol_table.functions {
                    if func.name.starts_with(&current_word) {
                        completions.push(CompletionItem {
                            label: func.name.clone(),
                            kind: Some(CompletionItemKind::FUNCTION),
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
                            insert_text: Some(format!("{}(${{1}})", func.name)),
                            insert_text_format: Some(InsertTextFormat::SNIPPET),
                            ..CompletionItem::default()
                        });
                    }
                }
                for type_keyword in &["String", "Number", "Boolean", "true", "false"] {
                    if type_keyword.starts_with(&current_word) {
                        completions.push(CompletionItem {
                            label: type_keyword.to_string(),
                            kind: Some(CompletionItemKind::KEYWORD),
                            ..CompletionItem::default()
                        });
                    }
                }
            }
            CompletionContext::Other => {
                let basic_keywords = [
                    "node", "nd", "fn", "text", "events", "choice", "when", "return", "break",
                ];
                for keyword in &basic_keywords {
                    if keyword.starts_with(&current_word) {
                        completions.push(CompletionItem {
                            label: keyword.to_string(),
                            kind: Some(CompletionItemKind::KEYWORD),
                            ..CompletionItem::default()
                        });
                    }
                }
            }
        }

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
        // TODO: 实现函数签名帮助
        Ok(None)
    }

    async fn goto_definition(
        &self,
        _params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        // TODO: 实现跳转到定义
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
                range: Range::new(Position::new(0, 0), Position::new(0, 0)), // TODO: 获取实际范围
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
        _params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        // TODO: 实现语义token高亮
        Ok(None)
    }
}
