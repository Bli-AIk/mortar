//! # error.rs
//!
//! # error.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! This file defines the parser-local error type used by the Mortar compiler. It keeps the syntax
//! parser's failure vocabulary small and structured so higher layers can attach spans, collect
//! diagnostics, and recover parsing progress in a uniform way.
//!
//! 这个文件定义了 Mortar 编译器语法解析器本地使用的错误类型。它把解析阶段的失败类型保持得既小
//! 又结构化，方便更上层统一附加 span、收集诊断并做错误恢复。

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: String },
    ExpectedIdentifier { found: String },
    ExpectedString { found: String },
    UnexpectedEOF,
    InvalidNumber(String),
    Custom(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "Expected {}, found {}", expected, found)
            }
            ParseError::ExpectedIdentifier { found } => {
                write!(f, "Expected identifier, found {}", found)
            }
            ParseError::ExpectedString { found } => {
                write!(f, "Expected string, found {}", found)
            }
            ParseError::UnexpectedEOF => write!(f, "Unexpected end of input"),
            ParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ParseError::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for ParseError {}
