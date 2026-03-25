//! # expression.rs
//!
//! # expression.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Parses Mortar expressions, conditions, assignments, and interpolated strings. It is
//! the grammar layer that turns token sequences into typed AST forms for comparisons, function
//! calls, literal values, and embedded string expressions.
//!
//! 负责解析 Mortar 的表达式、条件、赋值和值得插值的字符串。它是把 token 序列转换成
//! 比较、函数调用、字面量以及嵌入式字符串表达式等类型化 AST 结构的语法层。

use super::Parser;
use super::error::ParseError;
use crate::ast::{
    Arg, AssignValue, BinaryCondition, ComparisonOp, FuncCall, IfCondition, InterpolatedString,
    StringPart, UnaryCondition, UnaryOp, VarValue,
};
use crate::escape::{process_triple_quoted_string, unescape};
use crate::token::Token;

mod interpolation_helpers;

use interpolation_helpers::{
    handle_escape_sequence, handle_interpolated_brace, parse_identifier_condition,
};

pub trait ExpressionParser {
    fn parse_if_condition(&mut self) -> Result<IfCondition, ParseError>;
    fn parse_or_expression(&mut self) -> Result<IfCondition, ParseError>;
    fn parse_and_expression(&mut self) -> Result<IfCondition, ParseError>;
    fn parse_comparison_expression(&mut self) -> Result<IfCondition, ParseError>;
    fn parse_unary_expression(&mut self) -> Result<IfCondition, ParseError>;
    fn parse_primary_if_condition(&mut self) -> Result<IfCondition, ParseError>;
    fn peek_comparison_op(&self) -> Option<ComparisonOp>;

    fn parse_func_call(&mut self) -> Result<FuncCall, ParseError>;
    fn parse_arg(&mut self) -> Result<Arg, ParseError>;

    fn parse_assign_value(&mut self) -> Result<AssignValue, ParseError>;
    fn parse_var_value(&mut self) -> Result<VarValue, ParseError>;
    fn parse_type(&mut self) -> Result<String, ParseError>;

    fn parse_interpolated_string(&mut self, text: &str) -> Result<InterpolatedString, ParseError>;
    fn parse_expression_from_string(&mut self, expr_text: &str) -> Result<FuncCall, ParseError>;
    fn parse_simple_args(&mut self, args_text: &str) -> Result<Vec<Arg>, ParseError>;
}

impl<'a> ExpressionParser for Parser<'a> {
    fn parse_if_condition(&mut self) -> Result<IfCondition, ParseError> {
        self.parse_or_expression()
    }

    fn parse_or_expression(&mut self) -> Result<IfCondition, ParseError> {
        let mut left = self.parse_and_expression()?;

        while self.check(&Token::Or) {
            self.advance();
            let right = self.parse_and_expression()?;
            left = IfCondition::Binary(Box::new(BinaryCondition {
                left,
                operator: ComparisonOp::Or,
                right,
            }));
        }

        Ok(left)
    }

    fn parse_and_expression(&mut self) -> Result<IfCondition, ParseError> {
        let mut left = self.parse_comparison_expression()?;

        while self.check(&Token::And) {
            self.advance();
            let right = self.parse_comparison_expression()?;
            left = IfCondition::Binary(Box::new(BinaryCondition {
                left,
                operator: ComparisonOp::And,
                right,
            }));
        }

        Ok(left)
    }

    fn parse_comparison_expression(&mut self) -> Result<IfCondition, ParseError> {
        let mut left = self.parse_unary_expression()?;

        while let Some(op) = self.peek_comparison_op() {
            self.advance(); // consume operator
            let right = self.parse_unary_expression()?;
            left = IfCondition::Binary(Box::new(BinaryCondition {
                left,
                operator: op,
                right,
            }));
        }

        Ok(left)
    }

    fn parse_unary_expression(&mut self) -> Result<IfCondition, ParseError> {
        if self.check(&Token::Not) {
            self.advance();
            let operand = self.parse_unary_expression()?;
            return Ok(IfCondition::Unary(Box::new(UnaryCondition {
                operator: UnaryOp::Not,
                operand,
            })));
        }

        self.parse_primary_if_condition()
    }

    fn parse_primary_if_condition(&mut self) -> Result<IfCondition, ParseError> {
        // Handle parenthesized expressions
        if self.check(&Token::LeftParen) {
            self.advance();
            let cond = self.parse_if_condition()?;
            self.consume(&Token::RightParen, "Expected ')' after condition")?;
            return Ok(cond);
        }

        // Handle boolean literals
        if self.check(&Token::True) {
            self.advance();
            return Ok(IfCondition::Literal(true));
        }

        if self.check(&Token::False) {
            self.advance();
            return Ok(IfCondition::Literal(false));
        }

        // Handle identifiers (variables, enum members) and numbers
        let Some(token_info) = self.peek() else {
            return Err(ParseError::Custom(
                "Expected condition expression".to_string(),
            ));
        };
        match &token_info.token {
            Token::Identifier(name) => {
                // Look ahead to see if it's a function call
                if self.tokens.get(self.current + 1).map(|t| &t.token) == Some(&Token::LeftParen) {
                    let func_call = self.parse_func_call()?;
                    Ok(IfCondition::FuncCall(func_call))
                } else {
                    parse_identifier_condition(self, name.to_string())
                }
            }
            Token::Number(num) => {
                let num = num.to_string();
                self.advance();
                Ok(IfCondition::Identifier(num))
            }
            _ => Err(ParseError::Custom(
                "Expected condition expression".to_string(),
            )),
        }
    }

    fn peek_comparison_op(&self) -> Option<ComparisonOp> {
        match self.peek().map(|t| &t.token) {
            Some(Token::Greater) => Some(ComparisonOp::Greater),
            Some(Token::Less) => Some(ComparisonOp::Less),
            Some(Token::GreaterEqual) => Some(ComparisonOp::GreaterEqual),
            Some(Token::LessEqual) => Some(ComparisonOp::LessEqual),
            Some(Token::EqualEqual) => Some(ComparisonOp::Equal),
            Some(Token::NotEqual) => Some(ComparisonOp::NotEqual),
            _ => None,
        }
    }

    fn parse_func_call(&mut self) -> Result<FuncCall, ParseError> {
        let (name, name_span) = if let Some(token_info) = self.advance() {
            match &token_info.token {
                Token::Identifier(name) => {
                    (name.to_string(), Some((token_info.start, token_info.end)))
                }
                // Accept a few keyword-shaped builtins as callable names.
                Token::Wait => ("wait".to_string(), Some((token_info.start, token_info.end))),
                Token::Run => ("run".to_string(), Some((token_info.start, token_info.end))),
                Token::Action => (
                    "action".to_string(),
                    Some((token_info.start, token_info.end)),
                ),
                Token::Index => (
                    "index".to_string(),
                    Some((token_info.start, token_info.end)),
                ),
                Token::Duration => (
                    "duration".to_string(),
                    Some((token_info.start, token_info.end)),
                ),
                _ => {
                    return Err(ParseError::Custom("Expected function name".to_string()));
                }
            }
        } else {
            return Err(ParseError::Custom("Expected function name".to_string()));
        };

        self.consume(&Token::LeftParen, "Expected '('")?;

        let mut args = Vec::new();

        while !self.check(&Token::RightParen) && !self.is_at_end() {
            args.push(self.parse_arg()?);
            self.skip_optional_separators();

            if self.check(&Token::RightParen) {
                break;
            }
        }

        self.consume(&Token::RightParen, "Expected ')'")?;

        Ok(FuncCall {
            name,
            name_span,
            args,
        })
    }

    fn parse_arg(&mut self) -> Result<Arg, ParseError> {
        match self.peek().map(|t| &t.token) {
            Some(Token::String(s)) => {
                let s = unescape(s);
                self.advance();
                Ok(Arg::String(s))
            }
            Some(Token::TripleQuotedString(s)) => {
                let s = process_triple_quoted_string(s);
                self.advance();
                Ok(Arg::String(s))
            }
            Some(Token::Number(n)) => {
                let n = n
                    .parse::<f64>()
                    .map_err(|_| ParseError::InvalidNumber(n.to_string()))?;
                self.advance();
                Ok(Arg::Number(n))
            }
            Some(Token::True) => {
                self.advance();
                Ok(Arg::Boolean(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(Arg::Boolean(false))
            }
            Some(Token::Identifier(name)) => {
                // Look ahead to see if it's a function call
                if self.tokens.get(self.current + 1).map(|t| &t.token) == Some(&Token::LeftParen) {
                    Ok(Arg::FuncCall(Box::new(self.parse_func_call()?)))
                } else {
                    let name = name.to_string();
                    self.advance();
                    Ok(Arg::Identifier(name))
                }
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "argument".to_string(),
                found: format!("{:?}", self.peek().map(|t| &t.token)),
            }),
        }
    }

    fn parse_assign_value(&mut self) -> Result<AssignValue, ParseError> {
        match self.peek().map(|t| &t.token) {
            Some(Token::String(s)) => {
                let value = unescape(s);
                self.advance();
                Ok(AssignValue::String(value))
            }
            Some(Token::TripleQuotedString(s)) => {
                let value = process_triple_quoted_string(s);
                self.advance();
                Ok(AssignValue::String(value))
            }
            Some(Token::Number(n)) => {
                let value = n
                    .parse::<f64>()
                    .map_err(|_| ParseError::InvalidNumber(n.to_string()))?;
                self.advance();
                Ok(AssignValue::Number(value))
            }
            Some(Token::True) => {
                self.advance();
                Ok(AssignValue::Boolean(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(AssignValue::Boolean(false))
            }
            Some(Token::Identifier(name)) => {
                let first_name = name.to_string();
                self.advance();
                // Check for enum member access (EnumName.member)
                if self.check(&Token::Dot) {
                    self.advance(); // consume '.'
                    let member = self.consume_identifier("Expected enum member name after '.'")?;
                    Ok(AssignValue::EnumMember(first_name, member))
                } else {
                    Ok(AssignValue::Identifier(first_name))
                }
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "value (string, number, boolean, identifier, or enum member)".to_string(),
                found: format!("{:?}", self.peek().map(|t| &t.token)),
            }),
        }
    }

    fn parse_var_value(&mut self) -> Result<VarValue, ParseError> {
        match self.peek().map(|t| &t.token) {
            Some(Token::String(s)) => {
                let value = unescape(s);
                self.advance();
                Ok(VarValue::String(value))
            }
            Some(Token::TripleQuotedString(s)) => {
                let value = process_triple_quoted_string(s);
                self.advance();
                Ok(VarValue::String(value))
            }
            Some(Token::Number(n)) => {
                let value = n
                    .parse::<f64>()
                    .map_err(|_| ParseError::InvalidNumber(n.to_string()))?;
                self.advance();
                Ok(VarValue::Number(value))
            }
            Some(Token::True) => {
                self.advance();
                Ok(VarValue::Boolean(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(VarValue::Boolean(false))
            }
            Some(Token::Identifier(name)) => {
                let enum_name = name.to_string();
                self.advance();
                // Check for enum member access (EnumName.member)
                if self.check(&Token::Dot) {
                    self.advance(); // consume '.'
                    let member = self.consume_identifier("Expected enum member name after '.'")?;
                    Ok(VarValue::EnumMember(enum_name, member))
                } else {
                    Err(ParseError::Custom(format!(
                        "Unexpected identifier '{}' in variable value",
                        enum_name
                    )))
                }
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "value (string, number, boolean, or enum member)".to_string(),
                found: format!("{:?}", self.peek().map(|t| &t.token)),
            }),
        }
    }

    fn parse_type(&mut self) -> Result<String, ParseError> {
        match self.advance().map(|t| &t.token) {
            Some(Token::Identifier(type_name)) => Ok(type_name.to_string()),
            Some(Token::StringType) => Ok("String".to_string()),
            Some(Token::NumberType) => Ok("Number".to_string()),
            Some(Token::BooleanType) => Ok("Boolean".to_string()),
            _ => Err(ParseError::Custom("Expected type".to_string())),
        }
    }

    fn parse_interpolated_string(&mut self, text: &str) -> Result<InterpolatedString, ParseError> {
        let mut parts = Vec::new();
        let mut chars = text.chars().peekable();
        let mut current_text = String::new();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                handle_escape_sequence(&mut chars, &mut current_text);
                continue;
            }
            if ch == '{' {
                handle_interpolated_brace(self, &mut chars, &mut parts, &mut current_text)?;
                continue;
            }
            current_text.push(ch);
        }

        // Save any remaining text
        if !current_text.is_empty() {
            parts.push(StringPart::Text(current_text));
        }

        Ok(InterpolatedString { parts })
    }

    fn parse_expression_from_string(&mut self, expr_text: &str) -> Result<FuncCall, ParseError> {
        // Simple parsing of "function_name()" or "function_name(args)"
        let expr_text = expr_text.trim();

        if let Some(paren_pos) = expr_text.find('(') {
            let func_name = expr_text[..paren_pos].trim();
            let args_part = &expr_text[paren_pos + 1..];

            if !args_part.ends_with(')') {
                return Err(ParseError::Custom(
                    "Expected ')' at end of function call".to_string(),
                ));
            }

            let args_part = &args_part[..args_part.len() - 1].trim();
            let args = if args_part.is_empty() {
                Vec::new()
            } else {
                // For now, only support simple arguments (this could be expanded)
                self.parse_simple_args(args_part)?
            };

            Ok(FuncCall {
                name: func_name.to_string(),
                name_span: None, // We don't have precise span info from string parsing
                args,
            })
        } else {
            Err(ParseError::Custom(
                "Expression in interpolated string must be a function call".to_string(),
            ))
        }
    }

    fn parse_simple_args(&mut self, args_text: &str) -> Result<Vec<Arg>, ParseError> {
        let mut args = Vec::new();

        for arg in args_text.split(',') {
            let arg = arg.trim();
            if arg.starts_with('"') && arg.ends_with('"') {
                args.push(Arg::String(arg[1..arg.len() - 1].to_string()));
                continue;
            }
            if arg.chars().all(|c| c.is_ascii_digit() || c == '.') {
                let num = arg
                    .parse::<f64>()
                    .map_err(|_| ParseError::InvalidNumber(arg.to_string()))?;
                args.push(Arg::Number(num));
                continue;
            }
            args.push(Arg::Identifier(arg.to_string()));
        }

        Ok(args)
    }
}
