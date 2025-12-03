//! # parser.rs
//!
//! # parser.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Implements the parser for the Mortar language, converting a stream of tokens into an Abstract Syntax Tree (AST).
//!
//! 实现 Mortar 语言的解析器，将 token 流转换为抽象语法树 (AST)。
//!
//! The parser is a recursive descent parser that handles grammar rules for nodes, expressions, and control flow.
//!
//! 该解析器是一个递归下降解析器，处理节点、表达式和控制流的语法规则。
//!
//! ## Source File Overview
//!
//! ## 源文件概述
//!
//! This file defines the `Parser` struct and its methods for parsing different language constructs, as well as the public `ParseHandler`.
//!
//! 此文件定义了 `Parser` 结构体及其解析各种语言构造的方法，以及公共的 `ParseHandler`。

pub mod expression;
pub mod statement;
pub mod top_level;
pub mod error;

use top_level::TopLevelParser;

use crate::ast::Program;
use crate::diagnostics::{Diagnostic, DiagnosticCollector, DiagnosticKind, Severity};
use crate::token::{Token, TokenInfo};
use error::ParseError;

pub struct ParseHandler;

impl ParseHandler {
    pub fn parse_source_code(content: &str, verbose_lexer: bool) -> Result<Program, ParseError> {
        let tokens = if verbose_lexer {
            crate::token::lex_with_output(content)
                .into_iter()
                .map(|token| TokenInfo {
                    token,
                    start: 0, // We'll use better position tracking later
                    end: 0,
                    text: "",
                })
                .collect()
        } else {
            crate::token::tokenize(content)
        };

        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }

    pub fn parse_source_code_with_diagnostics(
        content: &str,
        file_name: String,
        verbose_lexer: bool,
    ) -> (Result<Program, ParseError>, DiagnosticCollector) {
        Self::parse_source_code_with_diagnostics_and_language(
            content,
            file_name,
            verbose_lexer,
            crate::Language::English,
        )
    }

    pub fn parse_source_code_with_diagnostics_and_language(
        content: &str,
        file_name: String,
        verbose_lexer: bool,
        language: crate::Language,
    ) -> (Result<Program, ParseError>, DiagnosticCollector) {
        let tokens = if verbose_lexer {
            crate::token::lex_with_output(content)
                .into_iter()
                .map(|token| TokenInfo {
                    token,
                    start: 0, // We'll use better position tracking later
                    end: 0,
                    text: "",
                })
                .collect()
        } else {
            crate::token::tokenize(content)
        };

        let mut parser = Parser::new(tokens);
        let mut diagnostics = DiagnosticCollector::new_with_language(file_name, language);

        let result = parser.parse_program();

        // If parsing failed, add parse error to diagnostics
        if let Err(ref parse_error) = result {
            let current_span = parser.get_current_span();
            diagnostics.add_diagnostic(Diagnostic {
                kind: DiagnosticKind::SyntaxError {
                    message: parse_error.to_string(),
                },
                severity: Severity::Error,
                span: current_span,
                message: parse_error.to_string(),
            });
        }

        // If parsing succeeded, run semantic analysis
        if let Ok(ref program) = result {
            diagnostics.analyze_program(program);
        }

        (result, diagnostics)
    }
}

pub struct Parser<'a> {
    pub(super) tokens: Vec<TokenInfo<'a>>,
    pub(super) current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<TokenInfo<'a>>) -> Self {
        Self { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub(super) fn peek(&self) -> Option<&TokenInfo<'_>> {
        self.tokens.get(self.current)
    }

    pub(super) fn advance(&mut self) -> Option<&TokenInfo<'_>> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1)
    }

    pub(super) fn get_current_span(&self) -> Option<(usize, usize)> {
        if let Some(token_info) = self.peek() {
            Some((token_info.start, token_info.end))
        } else if self.current > 0 {
            // If we're at the end, use the last token's position
            self.tokens
                .get(self.current - 1)
                .map(|token_info| (token_info.start, token_info.end))
        } else {
            None
        }
    }

    pub(super) fn check(&self, token: &Token) -> bool {
        if let Some(current_token) = self.peek() {
            std::mem::discriminant(&current_token.token) == std::mem::discriminant(token)
        } else {
            false
        }
    }

    pub(super) fn consume(
        &mut self,
        expected: &Token,
        _error_msg: &str,
    ) -> Result<&TokenInfo<'_>, ParseError> {
        if self.check(expected) {
            Ok(self.advance().unwrap())
        } else {
            let found = self.peek()
                .map(|t| format!("{}", t.token))
                .unwrap_or_else(|| "EOF".to_string());
            Err(ParseError::UnexpectedToken {
                expected: format!("{}", expected),
                found,
            })
        }
    }

    pub(super) fn consume_identifier(&mut self, _error_msg: &str) -> Result<String, ParseError> {
        if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                Ok(name.to_string())
            } else {
                Err(ParseError::ExpectedIdentifier {
                    found: format!("{}", token_info.token),
                })
            }
        } else {
             Err(ParseError::UnexpectedEOF)
        }
    }

    pub(super) fn consume_string(&mut self, _error_msg: &str) -> Result<String, ParseError> {
        if let Some(token_info) = self.advance() {
            if let Token::String(s) = &token_info.token {
                Ok(s.to_string())
            } else {
                Err(ParseError::ExpectedString {
                    found: format!("{}", token_info.token),
                })
            }
        } else {
            Err(ParseError::UnexpectedEOF)
        }
    }

    /// Skip optional separators (commas and semicolons)
    pub(super) fn skip_optional_separators(&mut self) {
        while let Some(token_info) = self.peek() {
            if matches!(token_info.token, Token::Comma | Token::Semicolon) {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip comments and optional separators
    pub(super) fn skip_comments_and_separators(&mut self) {
        loop {
            let mut skipped_something = false;

            // Skip comments
            while let Some(token_info) = self.peek() {
                if matches!(
                    token_info.token,
                    Token::SingleLineComment(_) | Token::MultiLineComment(_)
                ) {
                    self.advance();
                    skipped_something = true;
                } else {
                    break;
                }
            }

            // Skip separators
            while let Some(token_info) = self.peek() {
                if matches!(token_info.token, Token::Comma | Token::Semicolon) {
                    self.advance();
                    skipped_something = true;
                } else {
                    break;
                }
            }

            if !skipped_something {
                break;
            }
        }
    }
}
