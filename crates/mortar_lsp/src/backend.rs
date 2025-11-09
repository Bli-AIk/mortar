use std::sync::Arc;

use dashmap::DashMap;
use mortar_compiler::Language;
use ropey::Rope;
use tokio::sync::RwLock;
use tower_lsp_server::Client;
use tower_lsp_server::lsp_types::*;
use tracing::info;

use crate::analysis::SymbolTable;
use crate::files::Files;

#[derive(Clone)]
pub struct Backend {
    pub client: Client,
    pub files: Arc<RwLock<Files>>,
    pub documents: Arc<DashMap<Uri, (Rope, Option<i32>)>>,
    pub diagnostics: Arc<DashMap<Uri, Vec<Diagnostic>>>,
    pub symbol_tables: Arc<DashMap<Uri, SymbolTable>>,
    pub language: Arc<RwLock<Language>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            files: Arc::new(RwLock::new(Files::new())),
            documents: Arc::new(DashMap::new()),
            diagnostics: Arc::new(DashMap::new()),
            symbol_tables: Arc::new(DashMap::new()),
            language: Arc::new(RwLock::new(Language::English)), // Default to English
        }
    }

    /// Set the language for diagnostics
    pub async fn set_language(&self, language: Language) {
        let mut lang = self.language.write().await;
        *lang = language;
    }

    /// Get the current language
    pub async fn get_language(&self) -> Language {
        *self.language.read().await
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

    pub fn _position_to_offset(&self, rope: &Rope, position: Position) -> Option<usize> {
        let line_idx = position.line as usize;
        if line_idx >= rope.len_lines() {
            return None;
        }

        let line_start = rope.line_to_char(line_idx);
        let char_offset = position.character as usize;
        let line_content = rope.line(line_idx);

        if char_offset > line_content.len_chars() {
            return None;
        }

        Some(line_start + char_offset)
    }

    pub fn _offset_to_position(&self, rope: &Rope, offset: usize) -> Option<Position> {
        if offset > rope.len_chars() {
            return None;
        }

        let line_idx = rope.char_to_line(offset);
        let line_start = rope.line_to_char(line_idx);
        let char_offset = offset - line_start;

        Some(Position::new(line_idx as u32, char_offset as u32))
    }

    pub fn get_current_word(&self, line: &str, char_idx: usize) -> String {
        let chars: Vec<char> = line.chars().collect();

        if char_idx >= chars.len() {
            return String::new();
        }

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
}

#[path = "backend/diagnostics.rs"]
pub mod diagnostics;

#[path = "backend/completion.rs"]
mod completion;

#[path = "backend/semantic_tokens.rs"]
mod semantic_tokens;

#[path = "backend/document_analysis.rs"]
mod document_analysis;

#[path = "backend/lsp_handlers.rs"]
mod lsp_handlers;

pub use completion::CompletionContext;
pub use diagnostics::{convert_diagnostics_to_lsp, parse_with_diagnostics};
