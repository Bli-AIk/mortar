use crate::token::{Token, TokenInfo};
use crate::diagnostics::{DiagnosticCollector, Diagnostic, DiagnosticKind, Severity};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub body: Vec<TopLevel>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel {
    NodeDef(NodeDef),
    FunctionDecl(FunctionDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeDef {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub body: Vec<NodeStmt>,
    pub jump: Option<NodeJump>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeStmt {
    Text(String),
    Events(Vec<Event>),
    Choice(Vec<ChoiceItem>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeJump {
    Identifier(String, Option<(usize, usize)>),
    Return,
    Break,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub index: f64,
    pub action: EventAction,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventAction {
    pub call: FuncCall,
    pub chains: Vec<FuncCall>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoiceItem {
    pub text: String,
    pub condition: Option<Condition>,
    pub target: ChoiceDest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    Identifier(String),
    FuncCall(FuncCall),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChoiceDest {
    Identifier(String, Option<(usize, usize)>),
    Return,
    Break,
    NestedChoices(Vec<ChoiceItem>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncCall {
    pub name: String,
    pub name_span: Option<(usize, usize)>,
    pub args: Vec<Arg>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    String(String),
    Number(f64),
    Identifier(String),
    FuncCall(Box<FuncCall>),
}

pub struct ParseHandler;

impl ParseHandler {
    pub fn parse_source_code(content: &str, verbose_lexer: bool) -> Result<Program, String> {
        let tokens = if verbose_lexer {
            crate::token::lex_with_output(content)
                .into_iter()
                .enumerate()
                .map(|(_i, token)| TokenInfo {
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
        verbose_lexer: bool
    ) -> (Result<Program, String>, DiagnosticCollector) {
        let tokens = if verbose_lexer {
            crate::token::lex_with_output(content)
                .into_iter()
                .enumerate()
                .map(|(_i, token)| TokenInfo {
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
        let mut diagnostics = DiagnosticCollector::new(file_name);
        
        let result = parser.parse_program();
        
        // If parsing failed, add parse error to diagnostics
        if let Err(ref parse_error) = result {
            let current_span = parser.get_current_span();
            diagnostics.add_diagnostic(Diagnostic {
                kind: DiagnosticKind::SyntaxError {
                    message: parse_error.clone(),
                },
                severity: Severity::Error,
                span: current_span,
                message: parse_error.clone(),
            });
        }
        
        // If parsing succeeded, run semantic analysis
        if let Ok(ref program) = result {
            diagnostics.analyze_program(program);
        }
        
        (result, diagnostics)
    }
}

struct Parser<'a> {
    tokens: Vec<TokenInfo<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<TokenInfo<'a>>) -> Self {
        Self { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<&TokenInfo<'_>> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&TokenInfo<'_>> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1)
    }

    fn get_current_span(&self) -> Option<(usize, usize)> {
        if let Some(token_info) = self.peek() {
            Some((token_info.start, token_info.end))
        } else if self.current > 0 {
            // If we're at the end, use the last token's position
            self.tokens.get(self.current - 1)
                .map(|token_info| (token_info.start, token_info.end))
        } else {
            None
        }
    }

    fn check(&self, token: &Token) -> bool {
        if let Some(current_token) = self.peek() {
            std::mem::discriminant(&current_token.token) == std::mem::discriminant(token)
        } else {
            false
        }
    }

    fn consume(&mut self, expected: &Token, error_msg: &str) -> Result<&TokenInfo<'_>, String> {
        if self.check(expected) {
            Ok(self.advance().unwrap())
        } else {
            Err(format!(
                "{}: expected {:?}, found {:?}",
                error_msg,
                expected,
                self.peek().map(|t| &t.token)
            ))
        }
    }

    /// Skip optional separators (commas and semicolons)
    fn skip_optional_separators(&mut self) {
        while let Some(token_info) = self.peek() {
            if matches!(token_info.token, Token::Comma | Token::Semicolon) {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip comments and optional separators
    fn skip_comments_and_separators(&mut self) {
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

    fn parse_program(&mut self) -> Result<Program, String> {
        let mut body = Vec::new();

        while !self.is_at_end() {
            self.skip_comments_and_separators();

            if !self.is_at_end() {
                body.push(self.parse_top_level()?);
            }
        }

        Ok(Program { body })
    }

    fn parse_top_level(&mut self) -> Result<TopLevel, String> {
        self.skip_comments_and_separators();

        match self.peek().map(|t| &t.token) {
            Some(Token::Node) => Ok(TopLevel::NodeDef(self.parse_node_def()?)),
            Some(Token::Fn) => Ok(TopLevel::FunctionDecl(self.parse_function_decl()?)),
            _ => Err(format!("Expected 'node' or 'fn', found {:?}", self.peek().map(|t| &t.token))),
        }
    }

    fn parse_node_def(&mut self) -> Result<NodeDef, String> {
        self.consume(&Token::Node, "Expected 'node'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected identifier after 'node'".to_string());
            }
        } else {
            return Err("Expected identifier after 'node'".to_string());
        };

        self.consume(&Token::LeftBrace, "Expected '{'")?;

        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if !self.check(&Token::RightBrace) && !self.is_at_end() {
                body.push(self.parse_node_stmt()?);
                self.skip_optional_separators();
            }
        }

        self.consume(&Token::RightBrace, "Expected '}'")?;

        let jump = if self.check(&Token::Arrow) {
            Some(self.parse_node_jump()?)
        } else {
            None
        };

        Ok(NodeDef { name, name_span, body, jump })
    }

    fn parse_node_stmt(&mut self) -> Result<NodeStmt, String> {
        match self.peek().map(|t| &t.token) {
            Some(Token::Text) => Ok(NodeStmt::Text(self.parse_text_stmt()?)),
            Some(Token::Events) => Ok(NodeStmt::Events(self.parse_events_stmt()?)),
            Some(Token::Choice) => Ok(NodeStmt::Choice(self.parse_choice_stmt()?)),
            _ => Err(format!(
                "Expected 'text', 'events', or 'choice', found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
    }

    fn parse_text_stmt(&mut self) -> Result<String, String> {
        self.consume(&Token::Text, "Expected 'text'")?;
        self.consume(&Token::Colon, "Expected ':'")?;

        if let Some(token_info) = self.advance() {
            if let Token::String(text) = &token_info.token {
                Ok(text.to_string())
            } else {
                Err("Expected string after 'text:'".to_string())
            }
        } else {
            Err("Expected string after 'text:'".to_string())
        }
    }

    fn parse_events_stmt(&mut self) -> Result<Vec<Event>, String> {
        self.consume(&Token::Events, "Expected 'events'")?;
        self.consume(&Token::Colon, "Expected ':'")?;
        self.consume(&Token::LeftBracket, "Expected '['")?;

        let mut events = Vec::new();

        while !self.check(&Token::RightBracket) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if !self.check(&Token::RightBracket) && !self.is_at_end() {
                events.push(self.parse_event()?);
                self.skip_optional_separators();
            }
        }

        self.consume(&Token::RightBracket, "Expected ']'")?;
        Ok(events)
    }

    fn parse_event(&mut self) -> Result<Event, String> {
        let index = if let Some(token_info) = self.advance() {
            if let Token::Number(n) = &token_info.token {
                n.parse::<f64>().map_err(|_| "Invalid number")?
            } else {
                return Err("Expected number for event index".to_string());
            }
        } else {
            return Err("Expected number for event index".to_string());
        };

        // Skip optional comma or semicolon after event index
        self.skip_optional_separators();

        let action = self.parse_event_action()?;

        Ok(Event { index, action })
    }

    fn parse_event_action(&mut self) -> Result<EventAction, String> {
        let call = self.parse_func_call()?;
        let mut chains = Vec::new();

        while self.check(&Token::Dot) {
            self.advance(); // consume '.'
            chains.push(self.parse_func_call()?);
        }

        Ok(EventAction { call, chains })
    }

    fn parse_choice_stmt(&mut self) -> Result<Vec<ChoiceItem>, String> {
        self.consume(&Token::Choice, "Expected 'choice'")?;
        self.consume(&Token::Colon, "Expected ':'")?;
        self.consume(&Token::LeftBracket, "Expected '['")?;

        let mut items = Vec::new();

        while !self.check(&Token::RightBracket) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if !self.check(&Token::RightBracket) && !self.is_at_end() {
                items.push(self.parse_choice_item()?);
                self.skip_optional_separators();
            }
        }

        self.consume(&Token::RightBracket, "Expected ']'")?;
        Ok(items)
    }

    fn parse_choice_item(&mut self) -> Result<ChoiceItem, String> {
        // Parse choice text
        let text = if self.check(&Token::LeftParen) {
            self.advance(); // consume '('
            let text = if let Some(token_info) = self.advance() {
                if let Token::String(s) = &token_info.token {
                    s.to_string()
                } else {
                    return Err("Expected string in parentheses".to_string());
                }
            } else {
                return Err("Expected string in parentheses".to_string());
            };
            self.consume(&Token::RightParen, "Expected ')'")?;
            text
        } else if let Some(token_info) = self.advance() {
            if let Token::String(s) = &token_info.token {
                s.to_string()
            } else {
                return Err("Expected choice text".to_string());
            }
        } else {
            return Err("Expected choice text".to_string());
        };

        // Parse optional condition
        let condition = if self.check(&Token::When)
            || (self.check(&Token::Dot) && self.tokens.get(self.current + 1).map(|t| &t.token) == Some(&Token::When))
        {
            Some(self.parse_choice_cond()?)
        } else {
            None
        };

        // Parse target
        self.consume(&Token::Arrow, "Expected '->'")?;
        let target = self.parse_choice_dest()?;

        Ok(ChoiceItem {
            text,
            condition,
            target,
        })
    }

    fn parse_choice_cond(&mut self) -> Result<Condition, String> {
        if self.check(&Token::Dot) {
            self.advance(); // consume '.'
            self.consume(&Token::When, "Expected 'when'")?;
            self.consume(&Token::LeftParen, "Expected '('")?;
            let condition = self.parse_condition()?;
            self.consume(&Token::RightParen, "Expected ')'")?;
            Ok(condition)
        } else {
            self.consume(&Token::When, "Expected 'when'")?;
            self.parse_condition()
        }
    }

    fn parse_condition(&mut self) -> Result<Condition, String> {
        if let Some(token_info) = self.peek() {
            if let Token::Identifier(name) = &token_info.token {
                // Look ahead to see if it's a function call
                if self.tokens.get(self.current + 1).map(|t| &t.token) == Some(&Token::LeftParen) {
                    Ok(Condition::FuncCall(self.parse_func_call()?))
                } else {
                    let name = name.to_string();
                    self.advance();
                    Ok(Condition::Identifier(name))
                }
            } else {
                Err("Expected identifier or function call in condition".to_string())
            }
        } else {
            Err("Expected identifier or function call in condition".to_string())
        }
    }

    fn parse_choice_dest(&mut self) -> Result<ChoiceDest, String> {
        match self.peek().map(|t| &t.token) {
            Some(Token::Identifier(_name)) => {
                let token_info = self.advance().unwrap();
                let name = if let Token::Identifier(name) = &token_info.token {
                    name.to_string()
                } else {
                    unreachable!()
                };
                let span = Some((token_info.start, token_info.end));
                Ok(ChoiceDest::Identifier(name, span))
            }
            Some(Token::Return) => {
                self.advance();
                Ok(ChoiceDest::Return)
            }
            Some(Token::Break) => {
                self.advance();
                Ok(ChoiceDest::Break)
            }
            Some(Token::LeftBracket) => {
                self.advance(); // consume '['

                let mut items = Vec::new();

                while !self.check(&Token::RightBracket) && !self.is_at_end() {
                    self.skip_comments_and_separators();

                    if !self.check(&Token::RightBracket) && !self.is_at_end() {
                        items.push(self.parse_choice_item()?);
                        self.skip_optional_separators();
                    }
                }

                self.consume(&Token::RightBracket, "Expected ']'")?;
                Ok(ChoiceDest::NestedChoices(items))
            }
            _ => Err(format!(
                "Expected choice destination, found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
    }

    fn parse_node_jump(&mut self) -> Result<NodeJump, String> {
        self.consume(&Token::Arrow, "Expected '->'")?;

        match self.peek().map(|t| &t.token) {
            Some(Token::Identifier(_name)) => {
                let token_info = self.advance().unwrap();
                let name = if let Token::Identifier(name) = &token_info.token {
                    name.to_string()
                } else {
                    unreachable!()
                };
                let span = Some((token_info.start, token_info.end));
                Ok(NodeJump::Identifier(name, span))
            }
            Some(Token::Return) => {
                self.advance();
                Ok(NodeJump::Return)
            }
            Some(Token::Break) => {
                self.advance();
                Ok(NodeJump::Break)
            }
            _ => Err(format!(
                "Expected identifier, 'return', or 'break', found {:?}",
                self.peek().map(|t| &t.token)
            )),
        }
    }

    fn parse_function_decl(&mut self) -> Result<FunctionDecl, String> {
        self.consume(&Token::Fn, "Expected 'fn'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected function name".to_string());
            }
        } else {
            return Err("Expected function name".to_string());
        };

        self.consume(&Token::LeftParen, "Expected '('")?;

        let mut params = Vec::new();

        while !self.check(&Token::RightParen) && !self.is_at_end() {
            params.push(self.parse_param()?);
            self.skip_optional_separators();

            if self.check(&Token::RightParen) {
                break;
            }
        }

        self.consume(&Token::RightParen, "Expected ')'")?;

        let return_type = if self.check(&Token::Arrow) {
            self.advance(); // consume '->'
            Some(self.parse_type()?)
        } else {
            None
        };

        Ok(FunctionDecl {
            name,
            name_span,
            params,
            return_type,
        })
    }

    fn parse_param(&mut self) -> Result<Param, String> {
        let name = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                name.to_string()
            } else {
                return Err("Expected parameter name".to_string());
            }
        } else {
            return Err("Expected parameter name".to_string());
        };

        self.consume(&Token::Colon, "Expected ':'")?;

        let type_name = self.parse_type()?;

        Ok(Param { name, type_name })
    }

    fn parse_type(&mut self) -> Result<String, String> {
        match self.advance().map(|t| &t.token) {
            Some(Token::Identifier(type_name)) => Ok(type_name.to_string()),
            Some(Token::StringType) => Ok("String".to_string()),
            Some(Token::NumberType) => Ok("Number".to_string()),
            Some(Token::BooleanType) => Ok("Boolean".to_string()),
            _ => Err("Expected type".to_string()),
        }
    }

    fn parse_func_call(&mut self) -> Result<FuncCall, String> {
        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err("Expected function name".to_string());
            }
        } else {
            return Err("Expected function name".to_string());
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

        Ok(FuncCall { name, name_span, args })
    }

    fn parse_arg(&mut self) -> Result<Arg, String> {
        match self.peek().map(|t| &t.token) {
            Some(Token::String(s)) => {
                let s = s.to_string();
                self.advance();
                Ok(Arg::String(s))
            }
            Some(Token::Number(n)) => {
                let n = n.parse::<f64>().map_err(|_| "Invalid number")?;
                self.advance();
                Ok(Arg::Number(n))
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
            _ => Err(format!("Expected argument, found {:?}", self.peek().map(|t| &t.token))),
        }
    }
}
