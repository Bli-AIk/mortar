//! # lib.rs
//!
//! # lib.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! This crate is the umbrella entry point for the Mortar toolchain. It re-exports compiler and
//! LSP capabilities and adds a small programmatic CLI facade so external callers can compile Mortar
//! files without spawning the binary directly.
//!
//! 这个 crate 是 Mortar 工具链的总入口。它重新导出编译器和 LSP 能力，并额外提供一个很小的
//! 编程式 CLI 门面，让外部调用方无需直接启动二进制程序也能编译 Mortar 文件。

// Re-export all functionality from sub-crates
pub use mortar_compiler::*;

// Re-export CLI functionality for programmatic use
pub mod cli {
    // CLI main function is not re-exported as it's meant to be used as binary
    // But we can provide a programmatic interface
    use crate::{FileHandler, ParseHandler, Serializer};

    /// Compile a mortar file programmatically
    pub fn compile_file(input_path: &str, output_path: Option<&str>) -> Result<(), String> {
        compile_file_with_format(input_path, output_path, false)
    }

    /// Compile a mortar file programmatically with format option
    pub fn compile_file_with_format(
        input_path: &str,
        output_path: Option<&str>,
        pretty: bool,
    ) -> Result<(), String> {
        // Read source file
        let content = FileHandler::read_source_file(input_path)
            .map_err(|err| format!("Failed to read input file: {}", err))?;

        let program = ParseHandler::parse_source_code(&content, false)
            .map_err(|err| format!("Parse error: {}", err))?;

        // Generate .mortared file
        let output_path = output_path.unwrap_or(input_path);

        Serializer::save_to_file(&program, output_path, pretty)
            .map_err(|err| format!("Failed to generate .mortared file: {}", err))?;

        Ok(())
    }
}

// Re-export LSP functionality when available
pub mod lsp {
    pub use mortar_lsp::*;
}

// Convenience prelude module
pub mod prelude {
    pub use crate::*;
}
