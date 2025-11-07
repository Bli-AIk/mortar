use std::sync::Arc;

use dashmap::DashMap;
use mortar_compiler::ParseHandler;
use ropey::Rope;
use tokio::sync::{RwLock, oneshot};
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::lsp_types::*;
use tower_lsp_server::{Client, LanguageServer};
use tracing::{info};

use crate::analysis::{analyze_program, SymbolTable};
use crate::files::Files;

pub struct Backend {
    pub client: Client,
    pub files: Arc<RwLock<Files>>,
    pub documents: Arc<DashMap<Uri, (Rope, Option<i32>)>>, // (content, version)
    pub diagnostics: Arc<DashMap<Uri, Vec<Diagnostic>>>,
    pub symbol_tables: Arc<DashMap<Uri, SymbolTable>>,
    pub shutdown_signal: Arc<RwLock<Option<oneshot::Sender<()>>>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            files: Arc::new(RwLock::new(Files::new())),
            documents: Arc::new(DashMap::new()),
            diagnostics: Arc::new(DashMap::new()),
            symbol_tables: Arc::new(DashMap::new()),
            shutdown_signal: Arc::new(RwLock::new(None)),
        }
    }

    /// 清理所有缓存的数据
    pub async fn cleanup(&self) {
        info!("开始清理LSP服务器资源...");
        
        // 清理文档缓存
        let documents_count = self.documents.len();
        self.documents.clear();
        info!("清理了 {} 个文档", documents_count);

        // 清理诊断信息
        let diagnostics_count = self.diagnostics.len();
        self.diagnostics.clear();
        info!("清理了 {} 个诊断信息", diagnostics_count);

        // 清理符号表
        let symbols_count = self.symbol_tables.len();
        self.symbol_tables.clear();
        info!("清理了 {} 个符号表", symbols_count);

        // 清理文件管理器
        {
            let mut files = self.files.write().await;
            *files = Files::new();
        }
        
        info!("LSP服务器资源清理完成");
    }

    /// 简化的清理操作，避免异步阻塞
    pub fn cleanup_sync(&self) {
        info!("开始同步清理LSP服务器资源...");
        
        // 清理文档缓存
        let documents_count = self.documents.len();
        self.documents.clear();
        
        // 清理诊断信息
        let diagnostics_count = self.diagnostics.len();
        self.diagnostics.clear();

        // 清理符号表
        let symbols_count = self.symbol_tables.len();
        self.symbol_tables.clear();
        
        info!("同步清理完成: {} 个文档, {} 个诊断, {} 个符号表", 
              documents_count, diagnostics_count, symbols_count);
    }

    /// 发送关闭信号（同步版本）
    pub fn signal_shutdown_sync(&self) {
        // 使用 try_write 避免阻塞
        if let Ok(mut shutdown_signal) = self.shutdown_signal.try_write() {
            if let Some(sender) = shutdown_signal.take() {
                let _ = sender.send(());
                info!("发送了关闭信号");
            }
        } else {
            info!("无法获取关闭信号锁，跳过");
        }
    }

    /// 等待关闭信号
    pub async fn wait_for_shutdown(&self) -> oneshot::Receiver<()> {
        let (sender, receiver) = oneshot::channel();
        let mut shutdown_signal = self.shutdown_signal.write().await;
        *shutdown_signal = Some(sender);
        receiver
    }

    /// Analyze document content and generate diagnostic information
    async fn analyze_document(&self, uri: &Uri, content: &str) {
        let mut diagnostics = Vec::new();
        let mut symbol_table = SymbolTable::new();

        match ParseHandler::parse_source_code(content) {
            Ok(program) => {
                // 分析程序以获得符号表和潜在的语义错误
                match analyze_program(&program) {
                    Ok(table) => {
                        symbol_table = table;
                    }
                    Err(errors) => {
                        // 将分析错误转换为诊断信息
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
                }
            }
            Err(error) => {
                // 解析错误
                diagnostics.push(Diagnostic::new_simple(
                    Range::new(Position::new(0, 0), Position::new(0, 0)),
                    error,
                ));
            }
        }

        // 保存符号表
        self.symbol_tables.insert(uri.clone(), symbol_table);

        // 保存诊断信息
        self.diagnostics.insert(uri.clone(), diagnostics.clone());

        // 发布诊断信息到客户端
        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }

    /// 将位置转换为文档中的偏移量
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

    /// 将偏移量转换为文档中的位置
    fn _offset_to_position(&self, rope: &Rope, offset: usize) -> Option<Position> {
        if offset > rope.len_chars() {
            return None;
        }

        let line_idx = rope.char_to_line(offset);
        let line_start = rope.line_to_char(line_idx);
        let char_idx = offset - line_start;

        Some(Position::new(line_idx as u32, char_idx as u32))
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
            .log_message(MessageType::INFO, "Mortar LSP 服务器已初始化")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        info!("收到关闭请求，开始关闭LSP服务器...");
        
        // 使用非常简单的清理逻辑，避免任何可能的阻塞
        let documents_count = self.documents.len();
        let diagnostics_count = self.diagnostics.len();
        let symbols_count = self.symbol_tables.len();
        
        info!("关闭时状态: {} 文档, {} 诊断, {} 符号", 
              documents_count, diagnostics_count, symbols_count);

        // 快速清理，不等待
        self.documents.clear();
        self.diagnostics.clear();
        self.symbol_tables.clear();

        info!("LSP服务器关闭完成");
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;
        let version = Some(params.text_document.version);

        // 存储文档内容
        let rope = Rope::from_str(&content);
        self.documents.insert(uri.clone(), (rope, version));

        // 分析文档
        self.analyze_document(&uri, &content).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = Some(params.text_document.version);

        if let Some(change) = params.content_changes.into_iter().next() {
            let content = change.text;
            let rope = Rope::from_str(&content);
            self.documents.insert(uri.clone(), (rope, version));

            // 重新分析文档
            self.analyze_document(&uri, &content).await;
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
        let _position = params.text_document_position.position;

        // 获取当前文档
        let (_rope, _version) = match self.documents.get(uri) {
            Some(entry) => (entry.0.clone(), entry.1),
            None => return Ok(None),
        };

        // 获取符号表
        let symbol_table = match self.symbol_tables.get(uri) {
            Some(table) => table.clone(),
            None => return Ok(None),
        };

        let mut completions = Vec::new();

        // 添加关键字补全
        let keywords = [
            "node", "nd", "text", "events", "choice", "fn", "return", "break", "when",
        ];

        for keyword in &keywords {
            completions.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..CompletionItem::default()
            });
        }

        // 添加函数补全
        for func in &symbol_table.functions {
            completions.push(CompletionItem {
                label: func.name.clone(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(format!(
                    "fn {}({}){}", 
                    func.name,
                    func.params.iter().map(|p| format!("{}: {}", p.name, p.type_name)).collect::<Vec<_>>().join(", "),
                    func.return_type.as_ref().map(|t| format!(" -> {}", t)).unwrap_or_default()
                )),
                insert_text: Some(format!("{}(${{1}})", func.name)),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..CompletionItem::default()
            });
        }

        // 添加节点补全
        for node in &symbol_table.nodes {
            completions.push(CompletionItem {
                label: node.clone(),
                kind: Some(CompletionItemKind::CLASS),
                ..CompletionItem::default()
            });
        }

        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let _uri = &params.text_document_position_params.text_document.uri;
        let _position = params.text_document_position_params.position;

        // 这里可以添加悬停信息的实现
        // 目前返回基本的语言信息
        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "**Mortar 语言**\n\n用于游戏对话和文本事件系统的DSL。".to_string(),
            }),
            range: None,
        }))
    }

    async fn signature_help(&self, _params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        // TODO: 实现函数签名帮助
        Ok(None)
    }

    async fn goto_definition(&self, _params: GotoDefinitionParams) -> Result<Option<GotoDefinitionResponse>> {
        // TODO: 实现跳转到定义
        Ok(None)
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        let uri = &params.text_document.uri;
        
        // 获取符号表
        let symbol_table = match self.symbol_tables.get(uri) {
            Some(table) => table.clone(),
            None => return Ok(None),
        };

        let mut symbols = Vec::new();

        // 添加节点符号
        for node in &symbol_table.nodes {
            symbols.push(DocumentSymbol {
                name: node.clone(),
                detail: None,
                kind: SymbolKind::CLASS,
                tags: None,
                deprecated: Some(false),
                range: Range::new(Position::new(0, 0), Position::new(0, 0)), // TODO: 获取实际范围
                selection_range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                children: None,
            });
        }

        // 添加函数符号
        for func in &symbol_table.functions {
            symbols.push(DocumentSymbol {
                name: func.name.clone(),
                detail: Some(format!(
                    "fn {}({}){}", 
                    func.name,
                    func.params.iter().map(|p| format!("{}: {}", p.name, p.type_name)).collect::<Vec<_>>().join(", "),
                    func.return_type.as_ref().map(|t| format!(" -> {}", t)).unwrap_or_default()
                )),
                kind: SymbolKind::FUNCTION,
                tags: None,
                deprecated: Some(false),
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                selection_range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                children: None,
            });
        }

        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }

    async fn semantic_tokens_full(&self, _params: SemanticTokensParams) -> Result<Option<SemanticTokensResult>> {
        // TODO: 实现语义token高亮
        Ok(None)
    }
}