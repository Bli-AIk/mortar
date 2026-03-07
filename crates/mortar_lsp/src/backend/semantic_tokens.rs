use mortar_compiler::{Token, TokenInfo, tokenize};
use tower_lsp_server::lsp_types::*;

use crate::backend::Backend;

impl Backend {
    /// Analyze semantic tokens for syntax highlighting
    pub fn analyze_semantic_tokens(&self, content: &str) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();
        let mut last_line = 0u32;
        let mut last_column = 0u32;

        // Tokenize the entire document instead of processing line by line
        let compiler_tokens = tokenize(content);

        for (i, token_info) in compiler_tokens.iter().enumerate() {
            let token_type =
                self.get_semantic_token_type_with_context(&token_info.token, &compiler_tokens, i);

            // Calculate token line and column position
            let (token_line, token_column) =
                self.get_line_column_position(content, token_info.start);

            // Calculate token's UTF-16 length (instead of UTF-8 byte length)
            let token_text = &content[token_info.start..token_info.end];
            let length = token_text.encode_utf16().count() as u32;

            let delta_line = token_line - last_line;
            let delta_start = if delta_line == 0 {
                token_column - last_column
            } else {
                token_column
            };

            tokens.push(SemanticToken {
                delta_line,
                delta_start,
                length,
                token_type,
                token_modifiers_bitset: 0,
            });

            last_line = token_line;
            last_column = token_column;
        }

        tokens
    }

    /// Calculate line and column position from byte offset
    /// LSP uses UTF-16 code units for column positions
    fn get_line_column_position(&self, content: &str, offset: usize) -> (u32, u32) {
        let mut line = 0u32;
        let mut utf16_column = 0u32;

        // Use byte indices for iteration to ensure offset matching with tokenizer
        for (i, ch) in content.char_indices() {
            if i >= offset {
                break;
            }

            if ch == '\n' {
                line += 1;
                utf16_column = 0;
            } else {
                // UTF-16 code unit count: most characters take 1 unit, but some Unicode characters need 2 units
                utf16_column += ch.len_utf16() as u32;
            }
        }

        (line, utf16_column)
    }

    /// Check if a `choice` keyword appears before `bracket_index`, stopping at node boundaries.
    fn has_choice_before_bracket(&self, all_tokens: &[TokenInfo], bracket_index: usize) -> bool {
        for j in (0..bracket_index).rev() {
            match all_tokens[j].token {
                Token::Choice => return true,
                Token::LeftBrace | Token::RightBrace => break,
                _ => continue,
            }
        }
        false
    }

    /// Check if the current identifier is in a choice context (e.g., choice list)
    fn is_in_choice_context(&self, all_tokens: &[TokenInfo], current_index: usize) -> bool {
        let mut bracket_depth: i32 = 0;

        for i in (0..current_index).rev() {
            match all_tokens[i].token {
                Token::RightBracket => bracket_depth += 1,
                Token::LeftBracket if bracket_depth > 0 => bracket_depth -= 1,
                Token::LeftBracket => return self.has_choice_before_bracket(all_tokens, i),
                _ => {}
            }
        }

        false
    }

    /// Get semantic token type from compiler lexical token with context awareness
    fn get_semantic_token_type_with_context(
        &self,
        token: &Token,
        all_tokens: &[TokenInfo],
        current_index: usize,
    ) -> u32 {
        const KEYWORD: u32 = 0;
        const STRING: u32 = 1;
        const NUMBER: u32 = 2;
        const COMMENT: u32 = 3;
        const OPERATOR: u32 = 7;
        const PUNCTUATION: u32 = 8;

        match token {
            Token::SingleLineComment(_) | Token::MultiLineComment(_) => COMMENT,

            Token::Node
            | Token::Text
            | Token::Events
            | Token::Choice
            | Token::Fn
            | Token::Return
            | Token::Break
            | Token::When
            | Token::Let
            | Token::Const
            | Token::Pub
            | Token::Enum
            | Token::Branch
            | Token::If
            | Token::Else
            | Token::Event
            | Token::Run
            | Token::With
            | Token::Now
            | Token::Timeline
            | Token::Wait
            | Token::Index
            | Token::Action
            | Token::Duration
            | Token::StringType
            | Token::NumberType
            | Token::BooleanType
            | Token::True
            | Token::False => KEYWORD,

            Token::String(_) | Token::TripleQuotedString(_) | Token::InterpolatedString(_) => {
                STRING
            }

            Token::Number(_) => NUMBER,

            Token::Arrow => OPERATOR,

            Token::Colon
            | Token::Comma
            | Token::Semicolon
            | Token::Dot
            | Token::LeftBrace
            | Token::RightBrace
            | Token::LeftBracket
            | Token::RightBracket
            | Token::LeftParen
            | Token::RightParen
            | Token::Equals
            | Token::Less
            | Token::Greater
            | Token::LessEqual
            | Token::GreaterEqual
            | Token::EqualEqual
            | Token::NotEqual
            | Token::And
            | Token::Or
            | Token::Not => PUNCTUATION,

            Token::Identifier(_) => self.classify_identifier(all_tokens, current_index),

            Token::Error => KEYWORD,
        }
    }

    /// Classify an identifier token based on surrounding context.
    fn classify_identifier(&self, all_tokens: &[TokenInfo], current_index: usize) -> u32 {
        const FUNCTION: u32 = 4;
        const VARIABLE: u32 = 5;
        const METHOD: u32 = 6;

        let prev_token = current_index
            .checked_sub(1)
            .and_then(|i| all_tokens.get(i))
            .map(|ti| &ti.token);

        // Identifier after node/fn keyword is a definition name
        if matches!(prev_token, Some(Token::Node | Token::Fn)) {
            return FUNCTION;
        }

        // Identifier followed by left parenthesis is a function call
        if let Some(next) = all_tokens.get(current_index + 1)
            && matches!(next.token, Token::LeftParen)
        {
            return METHOD;
        }

        // Identifier after arrow is a node jump
        if matches!(prev_token, Some(Token::Arrow)) {
            return METHOD;
        }

        // Identifier after comma in a choice context is a node reference
        if matches!(prev_token, Some(Token::Comma))
            && self.is_in_choice_context(all_tokens, current_index)
        {
            return METHOD;
        }

        VARIABLE
    }
}
