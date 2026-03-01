//! # escape.rs
//!
//! ## Module Overview / 模块概述
//!
//! Provides escape sequence processing for Mortar string literals.
//!
//! 为 Mortar 字符串字面量提供转义序列处理。
//!
//! ## Supported Escape Sequences / 支持的转义序列
//!
//! - `\n` - Newline / 换行符
//! - `\t` - Tab / 制表符
//! - `\r` - Carriage return / 回车符
//! - `\\` - Backslash / 反斜杠
//! - `\"` - Double quote / 双引号
//! - `\'` - Single quote / 单引号

/// Unescapes a string by converting escape sequences to their actual characters.
///
/// # Arguments
/// * `s` - The input string potentially containing escape sequences
///
/// # Returns
/// The unescaped string with escape sequences converted to actual characters.
///
/// # Examples
/// ```
/// use mortar_compiler::escape::unescape;
///
/// assert_eq!(unescape(r"Hello\nWorld"), "Hello\nWorld");
/// assert_eq!(unescape(r"Tab\there"), "Tab\there");
/// assert_eq!(unescape(r#"Quote: \"test\""#), "Quote: \"test\"");
/// ```
pub fn unescape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                Some('0') => result.push('\0'),
                // For unknown escape sequences, keep them as-is
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Processes a triple-quoted string by:
/// 1. Removing leading/trailing blank lines
/// 2. Trimming common leading whitespace (dedent)
/// 3. Applying escape sequences
///
/// # Examples
/// ```
/// use mortar_compiler::escape::process_triple_quoted_string;
///
/// let input = "
///     Hello
///     World
/// ";
/// assert_eq!(process_triple_quoted_string(input), "Hello\nWorld");
/// ```
pub fn process_triple_quoted_string(s: &str) -> String {
    // Split into lines
    let lines: Vec<&str> = s.lines().collect();

    // Find first and last non-empty lines
    let first_non_empty = lines.iter().position(|line| !line.trim().is_empty());
    let last_non_empty = lines.iter().rposition(|line| !line.trim().is_empty());

    let (start, end) = match (first_non_empty, last_non_empty) {
        (Some(s), Some(e)) => (s, e),
        _ => return String::new(), // All empty lines
    };

    let content_lines = &lines[start..=end];

    // Calculate common leading whitespace
    let min_indent = content_lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

    // Dedent and join lines
    let dedented: Vec<String> = content_lines
        .iter()
        .map(|line| {
            if line.len() >= min_indent {
                line[min_indent..].to_string()
            } else {
                line.trim_start().to_string()
            }
        })
        .collect();

    let joined = dedented.join("\n");

    // Apply escape sequences
    unescape(&joined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_escapes() {
        assert_eq!(unescape(r"Hello\nWorld"), "Hello\nWorld");
        assert_eq!(unescape(r"Hello\tWorld"), "Hello\tWorld");
        assert_eq!(unescape(r"Hello\rWorld"), "Hello\rWorld");
    }

    #[test]
    fn test_quote_escapes() {
        assert_eq!(unescape(r#"Say \"Hello\""#), "Say \"Hello\"");
        assert_eq!(unescape(r"It\'s"), "It's");
    }

    #[test]
    fn test_backslash_escape() {
        assert_eq!(unescape(r"Path\\to\\file"), "Path\\to\\file");
    }

    #[test]
    fn test_null_escape() {
        assert_eq!(unescape(r"Null\0char"), "Null\0char");
    }

    #[test]
    fn test_no_escapes() {
        assert_eq!(unescape("Plain text"), "Plain text");
    }

    #[test]
    fn test_unknown_escape() {
        // Unknown escapes are kept as-is
        assert_eq!(unescape(r"\x"), "\\x");
    }

    #[test]
    fn test_trailing_backslash() {
        assert_eq!(unescape(r"ends with \"), "ends with \\");
    }

    #[test]
    fn test_multiple_escapes() {
        assert_eq!(unescape(r"Line1\nLine2\nLine3"), "Line1\nLine2\nLine3");
        assert_eq!(unescape(r"\t\t\tThree tabs"), "\t\t\tThree tabs");
    }

    #[test]
    fn test_mixed_escapes() {
        assert_eq!(
            unescape(r#"Name:\t\"Alice\"\nAge:\t25"#),
            "Name:\t\"Alice\"\nAge:\t25"
        );
    }
}
