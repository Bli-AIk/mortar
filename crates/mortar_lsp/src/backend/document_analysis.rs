//! # document_analysis.rs
//!
//! # document_analysis.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! This file runs background document analysis for the Mortar LSP backend. It coordinates parsing,
//! symbol-table construction, diagnostic publication, and the async boundary between the editor
//! request loop and the blocking compiler work.
//!
//! 这个文件负责 Mortar LSP backend 的后台文档分析。它协调解析、符号表构建、诊断发布，
//! 同时处理编辑器请求循环与阻塞式编译工作之间的异步边界。

use tokio;
use tower_lsp_server::lsp_types::*;

use crate::analysis::analyze_program;
use crate::backend::{Backend, parse_with_diagnostics};

impl Backend {
    /// Analyze document content and generate diagnostic information with language support
    pub async fn analyze_document(&self, uri: &Uri, content: &str) {
        let language = self.get_language().await;
        let file_name = uri.path().to_string();
        let content_owned = content.to_string();

        // Parse and analyze with diagnostics
        let (diagnostics, program_opt) = tokio::task::spawn_blocking(move || {
            parse_with_diagnostics(&content_owned, file_name, language)
        })
        .await
        .unwrap_or_else(|_| {
            // If the task panicked, create a simple error diagnostic
            (
                vec![Diagnostic::new_simple(
                    Range::new(Position::new(0, 0), Position::new(0, 0)),
                    crate::backend::i18n::get_lsp_text("analysis_task_failed", language)
                        .to_string(),
                )],
                None,
            )
        });

        // Update symbol table if program was parsed successfully
        if let Some(program) = program_opt
            && let Ok(symbol_table) =
                tokio::task::spawn_blocking(move || analyze_program(&program)).await
        {
            match symbol_table {
                Ok(table) => {
                    self.symbol_tables.insert(uri.clone(), table);
                }
                Err(_) => {
                    // Symbol analysis failed, but we already have parse diagnostics
                }
            }
        }

        // Store and publish diagnostics
        self.diagnostics.insert(uri.clone(), diagnostics.clone());

        let _ = self
            .client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }
}
