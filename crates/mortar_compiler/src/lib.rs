pub mod handler;
pub mod parser;
pub mod serializer;
pub mod token;
pub mod diagnostics;

#[cfg(test)]
mod tests;

pub use handler::file_handler::{FileError, FileHandler};
pub use parser::{
    ChoiceItem, Event, EventAction, NodeDef, NodeJump, NodeStmt, ParseHandler, Program, TopLevel,
};
pub use serializer::Serializer;
pub use token::{Token, TokenInfo, tokenize};
pub use diagnostics::{DiagnosticCollector, Diagnostic, DiagnosticKind, Severity};
