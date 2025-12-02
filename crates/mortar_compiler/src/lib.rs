pub mod deserializer;
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

pub use ast::ChoiceItem;
pub use ast::Event as ParserEvent;
pub use ast::EventAction;
pub use ast::NodeDef;
pub use ast::NodeJump;
pub use ast::NodeStmt;
pub use ast::Program;
pub use ast::TopLevel;
pub use deserializer::{
    Action, BranchCase, BranchDef, Choice, Condition, Constant, ContentItem, Deserializer, Enum,
    Event, EventDef, Function, IfCondition, IndexOverride, Metadata, MortaredData, Node, Param,
    Statement, StringPart, TimelineDef, TimelineStmt, Variable,
};
pub use diagnostics::{Diagnostic, DiagnosticCollector, DiagnosticKind, Severity};
pub use handler::file_handler::{FileError, FileHandler};
pub use parser::ParseHandler;
pub use serializer::Serializer;
pub use token::{Token, TokenInfo, tokenize};

pub mod ast;
#[cfg(test)]
mod tests;
