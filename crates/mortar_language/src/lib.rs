// Re-export all functionality from sub-crates
pub use mortar_compiler::*;

// Re-export CLI functionality for programmatic use
pub mod cli {
    // CLI main function is not re-exported as it's meant to be used as binary
    // But we can provide a programmatic interface
    use crate::{FileHandler, ParseHandler, Serializer};

    /// Compile a mortar file programmatically
    pub fn compile_file(input_path: &str, output_path: Option<&str>) -> Result<(), String> {
        // Read source file
        let content = FileHandler::read_source_file(input_path)
            .map_err(|_| "Failed to read input file".to_string())?;

        let program = ParseHandler::parse_source_code(&content)
            .map_err(|err| format!("Parse error: {}", err))?;

        // Generate .mortared file
        let output_path = output_path.unwrap_or(input_path);

        Serializer::save_to_file(&program, output_path)
            .map_err(|err| format!("Failed to generate .mortared file: {}", err))?;

        Ok(())
    }
}

// Re-export LSP functionality when available
pub mod lsp {
    pub use mortar_lsp::*;
}

// Re-export grammar functionality when available
pub mod grammar {
    pub use mortar_grammar::*;
}

// Convenience prelude module
pub mod prelude {
    pub use crate::*;
}
