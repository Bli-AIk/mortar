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

pub use deserializer::{
    Action, BranchCase, BranchDef, Choice, Condition, Constant, Deserializer, Enum, Event,
    EventDef, Function, IfCondition, Metadata, MortaredData, Node, Param, StringPart, Text,
    TimelineDef, TimelineStmt, Variable,
};
pub use diagnostics::{Diagnostic, DiagnosticCollector, DiagnosticKind, Severity};
pub use handler::file_handler::{FileError, FileHandler};
pub use parser::{
    ChoiceItem, Event as ParserEvent, EventAction, NodeDef, NodeJump, NodeStmt, ParseHandler,
    Program, TopLevel,
};
pub use serializer::Serializer;
pub use token::{Token, TokenInfo, tokenize};

#[cfg(test)]
mod tests;
