use mortar_compiler::{Token, tokenize};
use tower_lsp_server::lsp_types::*;

use crate::backend::Backend;

impl Backend {
    /// Analyze semantic tokens for syntax highlighting
    pub fn analyze_semantic_tokens(&self, content: &str) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();
        let mut last_line = 0u32;
        let mut last_column = 0u32;

        for (line_idx, line_content) in content.lines().enumerate() {
            let line_idx = line_idx as u32;

            let mut line_tokens = Vec::new();
            self.analyze_line_tokens_with_compiler(line_content, 0, &mut line_tokens);

            line_tokens.sort_by_key(|&(start, _length, _type)| start);

            for (start, length, token_type) in line_tokens {
                let delta_line = line_idx - last_line;
                let delta_start = if delta_line == 0 {
                    start - last_column
                } else {
                    start
                };

                tokens.push(SemanticToken {
                    delta_line,
                    delta_start,
                    length,
                    token_type,
                    token_modifiers_bitset: 0,
                });

                last_line = line_idx;
                last_column = start;
            }
        }

        tokens
    }

    /// Analyze lexical tokens for a line using compiler library
    fn analyze_line_tokens_with_compiler(
        &self,
        line_content: &str,
        offset: u32,
        line_tokens: &mut Vec<(u32, u32, u32)>,
    ) {
        let tokens = tokenize(line_content);

        for token_info in tokens {
            let start = token_info.start as u32 + offset;
            let length = (token_info.end - token_info.start) as u32;

            let token_type = self.get_semantic_token_type(&token_info.token);
            line_tokens.push((start, length, token_type));
        }
    }

    /// Get semantic token type from compiler lexical token
    fn get_semantic_token_type(&self, token: &Token) -> u32 {
        const KEYWORD: u32 = 0;
        const STRING: u32 = 1;
        const NUMBER: u32 = 2;
        const COMMENT: u32 = 3;
        const VARIABLE: u32 = 5;
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
            | Token::When => KEYWORD,

            Token::String(_) => STRING,

            Token::Number(_) => NUMBER,

            Token::Arrow => OPERATOR,

            Token::Colon
            | Token::Comma
            | Token::Dot
            | Token::LeftBrace
            | Token::RightBrace
            | Token::LeftBracket
            | Token::RightBracket
            | Token::LeftParen
            | Token::RightParen => PUNCTUATION,

            Token::Identifier(_) => VARIABLE,

            Token::Error => KEYWORD,
        }
    }
}
