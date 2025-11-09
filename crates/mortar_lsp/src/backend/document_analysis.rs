use mortar_compiler::ParseHandler;
use tokio;
use tower_lsp_server::lsp_types::*;

use crate::analysis::{SymbolTable, analyze_program};
use crate::backend::Backend;

impl Backend {
    /// Analyze document content and generate diagnostic information
    pub async fn analyze_document(&self, uri: &Uri, content: &str) {
        let mut diagnostics = Vec::new();
        let mut symbol_table = SymbolTable::new();

        let content_owned = content.to_string();
        match tokio::task::spawn_blocking(move || ParseHandler::parse_source_code(&content_owned, false))
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
                    format!("Parse error: {}", error),
                ));
            }
            Err(_) => {
                diagnostics.push(Diagnostic::new_simple(
                    Range::new(Position::new(0, 0), Position::new(0, 0)),
                    "Parse task failed".to_string(),
                ));
            }
        }

        self.diagnostics.insert(uri.clone(), diagnostics.clone());
        self.symbol_tables.insert(uri.clone(), symbol_table);

        let _ = self
            .client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }
}
