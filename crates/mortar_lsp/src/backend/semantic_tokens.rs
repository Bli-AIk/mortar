use mortar_compiler::{tokenize, Token};
use tower_lsp_server::lsp_types::*;

use crate::backend::Backend;

impl Backend {
    /// Analyze semantic tokens for syntax highlighting
    pub fn analyze_semantic_tokens(&self, content: &str) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();
        let mut last_line = 0u32;
        let mut last_column = 0u32;
        let mut in_multiline_comment = false;

        for (line_idx, line_content) in content.lines().enumerate() {
            let line_idx = line_idx as u32;
            
            let mut line_tokens = Vec::new();
            
            if in_multiline_comment {
                if let Some(end_pos) = line_content.find("*/") {
                    let comment_length = end_pos + 2;
                    line_tokens.push((0, comment_length as u32, 3u32));
                    in_multiline_comment = false;
                    
                    let remaining = &line_content[comment_length..];
                    self.analyze_line_tokens_with_compiler(remaining, comment_length as u32, &mut line_tokens);
                } else {
                    line_tokens.push((0, line_content.len() as u32, 3u32));
                }
            } else {
                self.analyze_line_tokens_with_compiler(line_content, 0, &mut line_tokens);
                
                if let Some(comment_start) = self.find_comment_outside_strings(line_content) {
                    if line_content[comment_start..].starts_with("/*") {
                        if let Some(end_pos) = line_content[comment_start + 2..].find("*/") {
                            let full_end_pos = comment_start + 2 + end_pos + 2;
                            line_tokens.retain(|(start, length, _)| {
                                let end = start + length;
                                end <= (comment_start as u32) || *start >= (full_end_pos as u32)
                            });
                            line_tokens.push((comment_start as u32, (full_end_pos - comment_start) as u32, 3u32));
                        } else {
                            in_multiline_comment = true;
                            let comment_length = line_content.len() - comment_start;
                            line_tokens.retain(|(start, length, _)| {
                                let end = start + length;
                                end <= (comment_start as u32)
                            });
                            line_tokens.push((comment_start as u32, comment_length as u32, 3u32));
                        }
                    }
                }
            }
            
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
    fn analyze_line_tokens_with_compiler(&self, line_content: &str, offset: u32, line_tokens: &mut Vec<(u32, u32, u32)>) {
        let tokens = tokenize(line_content);
        
        for token_info in tokens {
            let start = token_info.start as u32 + offset;
            let length = (token_info.end - token_info.start) as u32;
            
            let token_type = self.get_semantic_token_type(&token_info.token);
            line_tokens.push((start, length, token_type));
        }
    }

    /// Find comments outside of strings
    pub fn find_comment_outside_strings(&self, line: &str) -> Option<usize> {
        let mut in_string = false;
        let mut string_char = '\0';
        let mut i = 0;
        let chars: Vec<char> = line.chars().collect();
        
        while i < chars.len() {
            let ch = chars[i];
            
            if !in_string {
                if ch == '"' || ch == '\'' {
                    in_string = true;
                    string_char = ch;
                }
                else if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                    return Some(i);
                }
                else if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                    return Some(i);
                }
            } else {
                if ch == string_char && (i == 0 || chars[i - 1] != '\\') {
                    in_string = false;
                }
            }
            
            i += 1;
        }
        
        None
    }

    /// Find end position of multiline comments
    fn find_multiline_comment_end(&self, content: &str, start_line: usize, start_pos: usize) -> Option<(usize, usize)> {
        let lines: Vec<&str> = content.lines().collect();
        
        for line_idx in start_line..lines.len() {
            let line = lines[line_idx];
            let search_start = if line_idx == start_line { start_pos + 2 } else { 0 };
            
            if let Some(pos) = line[search_start..].find("*/") {
                return Some((line_idx, search_start + pos + 2));
            }
        }
        
        None
    }

    /// Get semantic token type from compiler lexical token
    fn get_semantic_token_type(&self, token: &Token) -> u32 {
        const KEYWORD: u32 = 0;
        const STRING: u32 = 1;
        const NUMBER: u32 = 2;
        const COMMENT: u32 = 3;
        const FUNCTION: u32 = 4;
        const VARIABLE: u32 = 5;
        const TYPE: u32 = 6;
        const OPERATOR: u32 = 7;
        const PUNCTUATION: u32 = 8;

        match token {
            Token::Node | Token::Text | Token::Events | Token::Choice | 
            Token::Fn | Token::Return | Token::Break | Token::When => KEYWORD,
            
            Token::String(_) => STRING,
            
            Token::Number(_) => NUMBER,
            
            Token::Arrow => OPERATOR,
            
            Token::Colon | Token::Comma | Token::Dot |
            Token::LeftBrace | Token::RightBrace |
            Token::LeftBracket | Token::RightBracket |
            Token::LeftParen | Token::RightParen => PUNCTUATION,
            
            Token::Identifier(_) => VARIABLE,
            
            Token::Error => KEYWORD,
        }
    }
}