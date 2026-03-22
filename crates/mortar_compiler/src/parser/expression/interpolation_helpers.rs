use std::iter::Peekable;
use std::str::Chars;

use super::{ExpressionParser, ParseError, Parser};
use crate::ast::{IfCondition, StringPart};
use crate::token::Token;

fn escape_char(ch: char) -> Option<char> {
    match ch {
        'n' => Some('\n'),
        't' => Some('\t'),
        'r' => Some('\r'),
        '\\' => Some('\\'),
        '"' => Some('"'),
        '\'' => Some('\''),
        '0' => Some('\0'),
        '{' => Some('{'),
        '}' => Some('}'),
        _ => None,
    }
}

fn collect_brace_expression(chars: &mut Peekable<Chars<'_>>) -> Result<String, ParseError> {
    let mut expr_text = String::new();
    let mut brace_count = 1i32;
    let mut in_string = false;
    let mut escape_next = false;

    for expr_ch in chars.by_ref() {
        if escape_next {
            expr_text.push(expr_ch);
            escape_next = false;
            continue;
        }
        if expr_ch == '\\' {
            expr_text.push(expr_ch);
            escape_next = true;
            continue;
        }
        if expr_ch == '"' {
            in_string = !in_string;
            expr_text.push(expr_ch);
            continue;
        }
        if !in_string && expr_ch == '{' {
            brace_count += 1;
        }
        if !in_string && expr_ch == '}' {
            brace_count -= 1;
            if brace_count == 0 {
                break;
            }
        }
        expr_text.push(expr_ch);
    }

    if brace_count != 0 {
        eprintln!(
            "DEBUG: brace_count={}, in_string={}, expr_text={:?}",
            brace_count, in_string, expr_text
        );
        return Err(ParseError::Custom(
            "Unmatched '{' in interpolated string".to_string(),
        ));
    }

    Ok(expr_text)
}

pub(super) fn handle_escape_sequence(chars: &mut Peekable<Chars<'_>>, current_text: &mut String) {
    let Some(&next_ch) = chars.peek() else {
        current_text.push('\\');
        return;
    };
    if let Some(escaped) = escape_char(next_ch) {
        chars.next();
        current_text.push(escaped);
    } else {
        current_text.push('\\');
    }
}

pub(super) fn handle_interpolated_brace(
    parser: &mut Parser<'_>,
    chars: &mut Peekable<Chars<'_>>,
    parts: &mut Vec<StringPart>,
    current_text: &mut String,
) -> Result<(), ParseError> {
    if !current_text.is_empty() {
        parts.push(StringPart::Text(current_text.clone()));
        current_text.clear();
    }
    let expr_text = collect_brace_expression(chars)?;
    let expr_trimmed = expr_text.trim();
    if expr_trimmed.contains('(') {
        let func_call = parser.parse_expression_from_string(&expr_text)?;
        parts.push(StringPart::Expression(func_call));
    } else {
        parts.push(StringPart::Placeholder(expr_trimmed.to_string()));
    }
    Ok(())
}

pub(super) fn parse_identifier_condition(
    parser: &mut Parser<'_>,
    name: String,
) -> Result<IfCondition, ParseError> {
    parser.advance();
    if parser.check(&Token::Dot) {
        parser.advance();
        let member = parser.consume_identifier("Expected enum member name after '.'")?;
        return Ok(IfCondition::EnumMember(name, member));
    }
    Ok(IfCondition::Identifier(name))
}
