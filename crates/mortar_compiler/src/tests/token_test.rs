//! # token_test.rs
//!
//! # token_test.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Tests for the lexical analyzer (lexer).
//!
//! 词法分析器（lexer）的测试。
//!
//! ## Source File Overview
//!
//! ## 源文件概述
//!
//! Verifies that the lexer correctly identifies tokens, keywords, and literals.
//!
//! 验证词法分析器能否正确识别 token、关键字和字面量。

use crate::token::{Token, tokenize};

#[test]
fn test_tokenize_let_keyword() {
    let input = "let";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].token, Token::Let));
}

#[test]
fn test_tokenize_const_keyword() {
    let input = "const";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].token, Token::Const));
}

#[test]
fn test_tokenize_pub_keyword() {
    let input = "pub";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].token, Token::Pub));
}

#[test]
fn test_tokenize_public_keyword() {
    let input = "public";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].token, Token::Pub));
}

#[test]
fn test_tokenize_enum_keyword() {
    let input = "enum";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].token, Token::Enum));
}

#[test]
fn test_tokenize_equals() {
    let input = "=";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].token, Token::Equals));
}

#[test]
fn test_tokenize_variable_declaration() {
    let input = "let player_name: String";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[0].token, Token::Let));
    assert!(matches!(tokens[1].token, Token::Identifier(_)));
    assert!(matches!(tokens[2].token, Token::Colon));
    assert!(matches!(tokens[3].token, Token::StringType));
}

#[test]
fn test_tokenize_variable_with_value() {
    let input = "let score: Number = 100";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 6);
    assert!(matches!(tokens[0].token, Token::Let));
    assert!(matches!(tokens[4].token, Token::Equals));
    assert!(matches!(tokens[5].token, Token::Number(_)));
}

#[test]
fn test_tokenize_constant_declaration() {
    let input = r#"pub const title: String = "Game""#;
    let tokens = tokenize(input);
    assert!(tokens.len() >= 7);
    assert!(matches!(tokens[0].token, Token::Pub));
    assert!(matches!(tokens[1].token, Token::Const));
    assert!(matches!(tokens[5].token, Token::Equals));
    assert!(matches!(tokens[6].token, Token::String(_)));
}

#[test]
fn test_tokenize_enum_definition() {
    let input = "enum State { active, paused }";
    let tokens = tokenize(input);
    assert!(tokens.len() >= 7);
    assert!(matches!(tokens[0].token, Token::Enum));
    assert!(matches!(tokens[1].token, Token::Identifier(_)));
    assert!(matches!(tokens[2].token, Token::LeftBrace));
    assert!(matches!(tokens[tokens.len() - 1].token, Token::RightBrace));
}

#[test]
fn test_tokenize_true_literal() {
    let input = "true";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].token, Token::True));
}

#[test]
fn test_tokenize_false_literal() {
    let input = "false";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].token, Token::False));
}

#[test]
fn test_tokenize_boolean_type() {
    let input = "Bool Boolean";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 2);
    assert!(matches!(tokens[0].token, Token::BooleanType));
    assert!(matches!(tokens[1].token, Token::BooleanType));
}

#[test]
fn test_tokenize_all_types() {
    let input = "String Number Boolean";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 3);
    assert!(matches!(tokens[0].token, Token::StringType));
    assert!(matches!(tokens[1].token, Token::NumberType));
    assert!(matches!(tokens[2].token, Token::BooleanType));
}

#[test]
fn test_tokenize_string_literal() {
    let input = r#""Hello World""#;
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    match &tokens[0].token {
        Token::String(s) => assert_eq!(*s, "Hello World"),
        _ => panic!("Expected String token"),
    }
}

#[test]
fn test_tokenize_single_quote_string() {
    let input = "'Hello'";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    match &tokens[0].token {
        Token::String(s) => assert_eq!(*s, "Hello"),
        _ => panic!("Expected String token"),
    }
}

#[test]
fn test_tokenize_number_integer() {
    let input = "42";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    match &tokens[0].token {
        Token::Number(n) => assert_eq!(*n, "42"),
        _ => panic!("Expected Number token"),
    }
}

#[test]
fn test_tokenize_number_float() {
    let input = "3.14";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    match &tokens[0].token {
        Token::Number(n) => assert_eq!(*n, "3.14"),
        _ => panic!("Expected Number token"),
    }
}

#[test]
fn test_tokenize_identifier() {
    let input = "my_variable";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    match &tokens[0].token {
        Token::Identifier(id) => assert_eq!(*id, "my_variable"),
        _ => panic!("Expected Identifier token"),
    }
}

#[test]
fn test_tokenize_single_line_comment() {
    let input = "// This is a comment";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].token, Token::SingleLineComment(_)));
}

#[test]
fn test_tokenize_multi_line_comment() {
    let input = "/* This is a\nmulti-line comment */";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].token, Token::MultiLineComment(_)));
}

#[test]
fn test_tokenize_all_keywords() {
    let input = "node nd text events choice fn function return break when";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 10);
    assert!(matches!(tokens[0].token, Token::Node));
    assert!(matches!(tokens[1].token, Token::Node));
    assert!(matches!(tokens[2].token, Token::Text));
    assert!(matches!(tokens[3].token, Token::Events));
    assert!(matches!(tokens[4].token, Token::Choice));
    assert!(matches!(tokens[5].token, Token::Fn));
    assert!(matches!(tokens[6].token, Token::Fn));
    assert!(matches!(tokens[7].token, Token::Return));
    assert!(matches!(tokens[8].token, Token::Break));
    assert!(matches!(tokens[9].token, Token::When));
}

#[test]
fn test_tokenize_punctuation() {
    let input = "-> : , ; . { } [ ] ( )";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 11);
    assert!(matches!(tokens[0].token, Token::Arrow));
    assert!(matches!(tokens[1].token, Token::Colon));
    assert!(matches!(tokens[2].token, Token::Comma));
    assert!(matches!(tokens[3].token, Token::Semicolon));
    assert!(matches!(tokens[4].token, Token::Dot));
    assert!(matches!(tokens[5].token, Token::LeftBrace));
    assert!(matches!(tokens[6].token, Token::RightBrace));
    assert!(matches!(tokens[7].token, Token::LeftBracket));
    assert!(matches!(tokens[8].token, Token::RightBracket));
    assert!(matches!(tokens[9].token, Token::LeftParen));
    assert!(matches!(tokens[10].token, Token::RightParen));
}

#[test]
fn test_token_display() {
    let input = "let score: Number = 100";
    let tokens = tokenize(input);

    // Test that Display trait works
    for token in tokens {
        let _display = format!("{}", token.token);
    }
}

#[test]
fn test_tokenize_interpolated_string() {
    let input = r#"$"Hello {name}""#;
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 1);
    match &tokens[0].token {
        Token::InterpolatedString(s) => assert_eq!(*s, "Hello {name}"),
        _ => panic!("Expected InterpolatedString token"),
    }
}

#[test]
fn test_tokenize_complex_program() {
    let input = r#"
        let health: Number = 100
        pub const title: String = "Game"
        enum State { playing, paused }
        
        node Start {
            text: "Hello"
        }
    "#;

    let tokens = tokenize(input);
    // Should have multiple tokens
    assert!(tokens.len() > 20);
}
