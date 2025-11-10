pub mod diagnostics;
pub mod handler;
pub mod parser;
pub mod serializer;
pub mod token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

pub use diagnostics::{Diagnostic, DiagnosticCollector, DiagnosticKind, Severity};
pub use handler::file_handler::{FileError, FileHandler};
pub use parser::{
    ChoiceItem, Event, EventAction, NodeDef, NodeJump, NodeStmt, ParseHandler, Program, TopLevel,
};
pub use serializer::Serializer;
pub use token::{Token, TokenInfo, tokenize};

#[cfg(test)]
mod tests;
