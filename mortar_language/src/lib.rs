// Re-export all functionality from mortar_compiler
pub use mortar_compiler::*;

// Optional: Add convenience functions for the language as a whole
pub mod prelude {
    pub use crate::*;
}