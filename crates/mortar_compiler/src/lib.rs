//! # lib.rs
//!
//! # lib.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! The main entry point for the `mortar_compiler` library.
//!
//! `mortar_compiler` 库的主入口点。
//!
//! It re-exports key components such as the AST, parser, lexer, and diagnostics system, making them available to external crates (like the CLI and LSP).
//!
//! 它重新导出了 AST、解析器、词法分析器和诊断系统等关键组件，供外部 crate（如 CLI 和 LSP）使用。
//!
//! ## Source File Overview
//!
//! ## 源文件概述
//!
//! This file defines the `Language` enum and exposes the public API of the compiler.
//!
//! 此文件定义了 `Language` 枚举并暴露了编译器的公共 API。

pub mod deserializer;
pub mod diagnostics;
pub mod escape;
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
