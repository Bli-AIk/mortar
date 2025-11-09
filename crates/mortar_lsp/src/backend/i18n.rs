use mortar_compiler::Language;
use std::collections::HashMap;
use std::sync::OnceLock;

static LSP_TEXTS: OnceLock<HashMap<&'static str, HashMap<Language, &'static str>>> =
    OnceLock::new();

fn init_lsp_texts() -> &'static HashMap<&'static str, HashMap<Language, &'static str>> {
    LSP_TEXTS.get_or_init(|| {
        let mut texts = HashMap::new();

        // Server startup messages
        texts.insert(
            "starting_lsp_server",
            [
                (Language::English, "Starting the Mortar LSP server..."),
                (Language::Chinese, "正在启动 Mortar LSP 服务器..."),
            ]
            .into(),
        );

        texts.insert(
            "lsp_server_ready",
            [
                (Language::English, "Mortar LSP server is ready"),
                (Language::Chinese, "Mortar LSP 服务器已就绪"),
            ]
            .into(),
        );

        // Client information
        texts.insert(
            "client_connected",
            [
                (Language::English, "Client connected"),
                (Language::Chinese, "客户端已连接"),
            ]
            .into(),
        );

        texts.insert(
            "language_set_to",
            [
                (Language::English, "Language set to"),
                (Language::Chinese, "语言设置为"),
            ]
            .into(),
        );

        texts.insert(
            "language_auto_detected",
            [
                (Language::English, "Language auto-detected from environment"),
                (Language::Chinese, "从环境变量自动检测语言"),
            ]
            .into(),
        );

        // Shutdown messages
        texts.insert(
            "shutdown_requested",
            [
                (Language::English, "Shutdown requested"),
                (Language::Chinese, "收到关闭请求"),
            ]
            .into(),
        );

        texts.insert(
            "cleanup_resources",
            [
                (Language::English, "Cleaning up LSP server resources"),
                (Language::Chinese, "清理 LSP 服务器资源"),
            ]
            .into(),
        );

        texts.insert(
            "cleanup_completed",
            [
                (Language::English, "LSP server resource cleanup completed"),
                (Language::Chinese, "LSP 服务器资源清理完成"),
            ]
            .into(),
        );

        texts.insert(
            "documents_cleaned",
            [
                (Language::English, "documents cleaned"),
                (Language::Chinese, "个文档已清理"),
            ]
            .into(),
        );

        texts.insert(
            "diagnostics_cleaned",
            [
                (Language::English, "diagnostic messages cleaned"),
                (Language::Chinese, "条诊断消息已清理"),
            ]
            .into(),
        );

        texts.insert(
            "symbols_cleaned",
            [
                (Language::English, "symbol tables cleaned"),
                (Language::Chinese, "个符号表已清理"),
            ]
            .into(),
        );

        // Error messages
        texts.insert(
            "analysis_task_failed",
            [
                (Language::English, "Analysis task failed"),
                (Language::Chinese, "分析任务失败"),
            ]
            .into(),
        );

        texts.insert(
            "unable_set_subscriber",
            [
                (Language::English, "Unable to set global default subscriber"),
                (Language::Chinese, "无法设置全局默认订阅者"),
            ]
            .into(),
        );

        // Command results
        texts.insert(
            "language_changed_success",
            [
                (Language::English, "Language changed successfully"),
                (Language::Chinese, "语言更改成功"),
            ]
            .into(),
        );

        texts.insert(
            "unsupported_language_error",
            [
                (Language::English, "Unsupported language. Use 'en' or 'zh'"),
                (Language::Chinese, "不支持的语言。请使用 'en' 或 'zh'"),
            ]
            .into(),
        );

        texts.insert(
            "invalid_command_arguments",
            [
                (
                    Language::English,
                    "Invalid arguments for setLanguage command",
                ),
                (Language::Chinese, "setLanguage 命令的参数无效"),
            ]
            .into(),
        );

        texts
    })
}

pub fn get_lsp_text(key: &str, language: Language) -> &'static str {
    let texts = init_lsp_texts();
    texts
        .get(key)
        .and_then(|map| map.get(&language))
        .unwrap_or_else(|| {
            // Fall back to English if the corresponding language text is not found
            texts
                .get(key)
                .and_then(|map| map.get(&Language::English))
                .unwrap_or(&"")
        })
}

/// Detect language from environment variables, similar to CLI implementation
pub fn detect_system_language() -> Language {
    // Check environment variables to determine system language
    if let Ok(lang) = std::env::var("LANG") {
        if lang.contains("zh") || lang.contains("cn") || lang.contains("CN") {
            return Language::Chinese;
        }
    }

    // Check other common language environment variables
    for env_var in &["LC_ALL", "LC_MESSAGES", "LANGUAGE"] {
        if let Ok(lang) = std::env::var(env_var) {
            if lang.contains("zh") || lang.contains("cn") || lang.contains("CN") {
                return Language::Chinese;
            }
        }
    }

    Language::English
}

/// Parse language from command line arguments or environment
pub fn parse_language_from_args() -> Option<Language> {
    let args: Vec<String> = std::env::args().collect();

    // Check for --lang or -L flag
    if let Some(pos) = args.iter().position(|arg| arg == "--lang" || arg == "-L") {
        if let Some(lang_str) = args.get(pos + 1) {
            return match lang_str.as_str() {
                "en" | "english" => Some(Language::English),
                "zh" | "chinese" => Some(Language::Chinese),
                _ => None,
            };
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lsp_text() {
        // Test English
        assert_eq!(
            get_lsp_text("starting_lsp_server", Language::English),
            "Starting the Mortar LSP server..."
        );

        // Test Chinese
        assert_eq!(
            get_lsp_text("starting_lsp_server", Language::Chinese),
            "正在启动 Mortar LSP 服务器..."
        );

        // Test fallback
        assert_eq!(get_lsp_text("nonexistent_key", Language::Chinese), "");
    }

    #[test]
    fn test_detect_system_language() {
        // This test depends on environment, so we just ensure it doesn't panic
        let _language = detect_system_language();
    }
}
