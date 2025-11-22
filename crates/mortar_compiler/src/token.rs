use logos::Logos;
use owo_colors::OwoColorize;
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
// Ignore whitespace
#[logos(skip r"[ \t\r\n]+")]
pub enum Token<'a> {
    #[allow(dead_code)]
    Error,

    // region Comments
    #[regex(r"//[^\n]*", |lex| lex.slice())]
    SingleLineComment(&'a str),

    #[regex(r"/\*([^*]|\*[^/])*\*/", |lex| lex.slice())]
    MultiLineComment(&'a str),
    // endregion

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
    #[token("function")]
    Fn,
    #[token("return")]
    Return,
    #[token("break")]
    Break,
    #[token("when")]
    When,

    // Variable and constant keywords
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("pub")]
    #[token("public")]
    Pub,
    #[token("enum")]
    Enum,

    // Branch interpolation keyword
    #[token("branch")]
    Branch,

    // Control flow keywords
    #[token("if")]
    If,
    #[token("else")]
    Else,

    // Performance system keywords
    #[token("event")]
    Event,
    #[token("run")]
    Run,
    #[token("with")]
    With,
    #[token("ref")]
    Ref,
    #[token("timeline")]
    #[token("tl")]
    Timeline,
    #[token("wait")]
    Wait,
    #[token("index")]
    Index,
    #[token("action")]
    Action,
    #[token("duration")]
    Duration,

    // Type keywords
    #[token("String")]
    StringType,
    #[token("Number")]
    NumberType,
    #[token("Boolean")]
    #[token("Bool")]
    BooleanType,

    // Boolean literals
    #[token("true")]
    True,
    #[token("false")]
    False,
    // endregion

    // region Operators & Punctuation
    #[token("->")]
    Arrow,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
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
    #[token("=")]
    Equals,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,
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

    // Interpolated string: $"text {expression} more text"
    // Using callback to properly handle nested quotes
    #[token("$\"", lex_interpolated_string)]
    InterpolatedString(&'a str),

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

            SingleLineComment(s) => write!(f, "{}", s),
            MultiLineComment(s) => write!(f, "{}", s),

            Node => write!(f, "node"),
            Text => write!(f, "text"),
            Events => write!(f, "events"),
            Choice => write!(f, "choice"),
            Fn => write!(f, "fn"),
            Return => write!(f, "return"),
            Break => write!(f, "break"),
            When => write!(f, "when"),
            Let => write!(f, "let"),
            Const => write!(f, "const"),
            Pub => write!(f, "pub"),
            Enum => write!(f, "enum"),
            Branch => write!(f, "branch"),
            If => write!(f, "if"),
            Else => write!(f, "else"),
            Event => write!(f, "event"),
            Run => write!(f, "run"),
            With => write!(f, "with"),
            Ref => write!(f, "ref"),
            Timeline => write!(f, "timeline"),
            Wait => write!(f, "wait"),
            Index => write!(f, "index"),
            Action => write!(f, "action"),
            Duration => write!(f, "duration"),

            StringType => write!(f, "String"),
            NumberType => write!(f, "Number"),
            BooleanType => write!(f, "Boolean"),
            True => write!(f, "true"),
            False => write!(f, "false"),

            Arrow => write!(f, "->"),
            Colon => write!(f, ":"),
            Comma => write!(f, ","),
            Semicolon => write!(f, ";"),
            Dot => write!(f, "."),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            LeftBracket => write!(f, "["),
            RightBracket => write!(f, "]"),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            Equals => write!(f, "="),
            Less => write!(f, "<"),
            Greater => write!(f, ">"),
            LessEqual => write!(f, "<="),
            GreaterEqual => write!(f, ">="),
            EqualEqual => write!(f, "=="),
            NotEqual => write!(f, "!="),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
            Not => write!(f, "!"),

            String(s) => write!(f, "\"{}\"", s),
            InterpolatedString(s) => write!(f, "$\"{}\"", s),
            Number(s) => write!(f, "{}", s),
            Identifier(s) => write!(f, "{}", s),
        }
    }
}

fn lex_interpolated_string<'a>(lex: &mut logos::Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let start = lex.span().end;
    let source = lex.source();
    let bytes = source.as_bytes();

    let mut pos = start;
    let mut depth = 0;
    let mut in_string_literal = false;
    let mut escape_next = false;

    while pos < bytes.len() {
        let ch = bytes[pos] as char;
        pos += 1;

        if escape_next {
            escape_next = false;
            continue;
        }

        if ch == '\\' {
            escape_next = true;
            continue;
        }

        if ch == '"' && depth == 0 && !in_string_literal {
            // Found the closing quote
            lex.bump(pos - start);
            let content_start = start;
            let content_end = pos - 1;
            return Some(&source[content_start..content_end]);
        }

        if ch == '"' {
            in_string_literal = !in_string_literal;
            continue;
        }

        if !in_string_literal {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                if depth > 0 {
                    depth -= 1;
                }
            }
        }
    }

    None
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

#[allow(dead_code)]
pub(crate) fn lex_silent(input: &str) -> Vec<Token<'_>> {
    let lex = Token::lexer(input);
    let mut tokens = Vec::new();

    for result in lex {
        match result {
            Ok(token) => {
                tokens.push(token);
            }
            Err(_) => {
                break;
            }
        }
    }

    tokens
}
