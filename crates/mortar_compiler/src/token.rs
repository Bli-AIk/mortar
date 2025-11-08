use logos::Logos;
use owo_colors::OwoColorize;
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
// Ignore whitespace
#[logos(skip r"[ \t\r\n]+")]
// Single line comment
#[logos(skip r"//[^\n]*")]
// Multi-line comments
#[logos(skip r"/\*([^*]|\*[^/])*\*/")]
pub enum Token<'a> {
    #[allow(dead_code)]
    Error,

    // region Keywords
    #[token("node")]
    #[token("nd")]
    Node,
    #[token("text")]
    Text,
    #[token("events")]
    Events,
    #[token("choice")]
    Choice,
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,
    #[token("break")]
    Break,
    #[token("when")]
    When,
    // endregion

    // region Operators & Punctuation
    #[token("->")]
    Arrow,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    // endregion

    // region Literals
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        &s[1..s.len()-1]
    })]
    #[regex(r#"'([^'\\]|\\.)*'"#, |lex| {
        let s = lex.slice();
        &s[1..s.len()-1]
    })]
    String(&'a str),

    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number(&'a str),

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Identifier(&'a str),
    // endregion
}

/// Lexical analysis result containing token information and position
#[derive(Debug, Clone)]
pub struct TokenInfo<'a> {
    pub token: Token<'a>,
    pub start: usize,
    pub end: usize,
    pub text: &'a str,
}

/// Public lexical analysis interface for LSP and other external components
pub fn tokenize(input: &str) -> Vec<TokenInfo<'_>> {
    use logos::Logos;
    
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();
    
    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(token) => {
                let span = lexer.span();
                tokens.push(TokenInfo {
                    token,
                    start: span.start,
                    end: span.end,
                    text: &input[span.start..span.end],
                });
            }
            Err(_) => {
                let span = lexer.span();
                tokens.push(TokenInfo {
                    token: Token::Error,
                    start: span.start,
                    end: span.end,
                    text: &input[span.start..span.end],
                });
            }
        }
    }
    
    tokens
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Error => write!(f, "Error"),

            Node => write!(f, "node"),
            Text => write!(f, "text"),
            Events => write!(f, "events"),
            Choice => write!(f, "choice"),
            Fn => write!(f, "fn"),
            Return => write!(f, "return"),
            Break => write!(f, "break"),
            When => write!(f, "when"),

            Arrow => write!(f, "->"),
            Colon => write!(f, ":"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            LeftBracket => write!(f, "["),
            RightBracket => write!(f, "]"),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),

            String(s) => write!(f, "\"{}\"", s),
            Number(s) => write!(f, "{}", s),
            Identifier(s) => write!(f, "{}", s),
        }
    }
}

pub(crate) fn lex_with_output(input: &str) -> Vec<Token<'_>> {
    let lex = Token::lexer(input);
    let mut tokens = Vec::new();

    println!();
    println!("{}", "(Mortar) Lexer output:".green());

    for result in lex {
        match result {
            Ok(token) => {
                print!("{:?} ", token);
                tokens.push(token);
            }
            Err(_) => {
                println!("{}", "Lexer error encountered!".red());
                break;
            }
        }
    }

    println!("\n");
    tokens
}
