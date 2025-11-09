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
            (vec![Diagnostic::new_simple(
                Range::new(Position::new(0, 0), Position::new(0, 0)),
                "Analysis task failed".to_string(),
            )], None)
        });

        // Update symbol table if program was parsed successfully
        if let Some(program) = program_opt {
            if let Ok(symbol_table) = tokio::task::spawn_blocking(move || analyze_program(&program)).await {
                match symbol_table {
                    Ok(table) => {
                        self.symbol_tables.insert(uri.clone(), table);
                    }
                    Err(_) => {
                        // Symbol analysis failed, but we already have parse diagnostics
                    }
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
