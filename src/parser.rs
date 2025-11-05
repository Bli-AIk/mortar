use crate::token::Token;

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
    Identifier(String),
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
    Identifier(String),
    Return,
    Break,
    NestedChoices(Vec<ChoiceItem>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
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
    pub fn parse_source_code(content: &str) -> Result<Program, String> {
        let tokens = crate::token::lex_with_output(content);
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }
}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, current: 0 }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token<'_>> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token<'_>> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1)
    }

    fn check(&self, token: &Token) -> bool {
        if let Some(current_token) = self.peek() {
            std::mem::discriminant(current_token) == std::mem::discriminant(token)
        } else {
            false
        }
    }

    fn consume(&mut self, expected: &Token, error_msg: &str) -> Result<&Token<'_>, String> {
        if self.check(expected) {
            Ok(self.advance().unwrap())
        } else {
            Err(format!(
                "{}: expected {:?}, found {:?}",
                error_msg,
                expected,
                self.peek()
            ))
        }
    }

    fn parse_program(&mut self) -> Result<Program, String> {
        let mut body = Vec::new();

        while !self.is_at_end() {
            body.push(self.parse_top_level()?);
        }

        Ok(Program { body })
    }

    fn parse_top_level(&mut self) -> Result<TopLevel, String> {
        match self.peek() {
            Some(Token::Node) => Ok(TopLevel::NodeDef(self.parse_node_def()?)),
            Some(Token::Fn) => Ok(TopLevel::FunctionDecl(self.parse_function_decl()?)),
            _ => Err(format!("Expected 'node' or 'fn', found {:?}", self.peek())),
        }
    }

    fn parse_node_def(&mut self) -> Result<NodeDef, String> {
        self.consume(&Token::Node, "Expected 'node'")?;

        let name = if let Some(Token::Identifier(name)) = self.advance() {
            name.to_string()
        } else {
            return Err("Expected identifier after 'node'".to_string());
        };

        self.consume(&Token::LeftBrace, "Expected '{'")?;

        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            // Skip optional commas
            if self.check(&Token::Comma) {
                self.advance();
                continue;
            }

            body.push(self.parse_node_stmt()?);
        }

        self.consume(&Token::RightBrace, "Expected '}'")?;

        let jump = if self.check(&Token::Arrow) {
            Some(self.parse_node_jump()?)
        } else {
            None
        };

        Ok(NodeDef { name, body, jump })
    }

    fn parse_node_stmt(&mut self) -> Result<NodeStmt, String> {
        match self.peek() {
            Some(Token::Text) => Ok(NodeStmt::Text(self.parse_text_stmt()?)),
            Some(Token::Events) => Ok(NodeStmt::Events(self.parse_events_stmt()?)),
            Some(Token::Choice) => Ok(NodeStmt::Choice(self.parse_choice_stmt()?)),
            _ => Err(format!(
                "Expected 'text', 'events', or 'choice', found {:?}",
                self.peek()
            )),
        }
    }

    fn parse_text_stmt(&mut self) -> Result<String, String> {
        self.consume(&Token::Text, "Expected 'text'")?;
        self.consume(&Token::Colon, "Expected ':'")?;

        if let Some(Token::String(text)) = self.advance() {
            Ok(text.to_string())
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
            events.push(self.parse_event()?);

            // Events are not separated by commas in the Mortar syntax
            // Each event is on its own line
        }

        self.consume(&Token::RightBracket, "Expected ']'")?;
        Ok(events)
    }

    fn parse_event(&mut self) -> Result<Event, String> {
        let index = if let Some(Token::Number(n)) = self.advance() {
            n.parse::<f64>().map_err(|_| "Invalid number")?
        } else {
            return Err("Expected number for event index".to_string());
        };

        self.consume(&Token::Comma, "Expected ',' after event index")?;

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
            items.push(self.parse_choice_item()?);

            // Choices can be separated by commas, but they are optional
            if self.check(&Token::Comma) {
                self.advance();
            }
        }

        self.consume(&Token::RightBracket, "Expected ']'")?;
        Ok(items)
    }

    fn parse_choice_item(&mut self) -> Result<ChoiceItem, String> {
        // Parse choice text
        let text = if self.check(&Token::LeftParen) {
            self.advance(); // consume '('
            let text = if let Some(Token::String(s)) = self.advance() {
                s.to_string()
            } else {
                return Err("Expected string in parentheses".to_string());
            };
            self.consume(&Token::RightParen, "Expected ')'")?;
            text
        } else if let Some(Token::String(s)) = self.advance() {
            s.to_string()
        } else {
            return Err("Expected choice text".to_string());
        };

        // Parse optional condition
        let condition = if self.check(&Token::When)
            || (self.check(&Token::Dot) && self.tokens.get(self.current + 1) == Some(&Token::When))
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
        if let Some(Token::Identifier(name)) = self.peek() {
            // Look ahead to see if it's a function call
            if self.tokens.get(self.current + 1) == Some(&Token::LeftParen) {
                Ok(Condition::FuncCall(self.parse_func_call()?))
            } else {
                let name = name.to_string();
                self.advance();
                Ok(Condition::Identifier(name))
            }
        } else {
            Err("Expected identifier or function call in condition".to_string())
        }
    }

    fn parse_choice_dest(&mut self) -> Result<ChoiceDest, String> {
        match self.peek() {
            Some(Token::Identifier(name)) => {
                let name = name.to_string();
                self.advance();
                Ok(ChoiceDest::Identifier(name))
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
                    items.push(self.parse_choice_item()?);

                    if self.check(&Token::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }

                self.consume(&Token::RightBracket, "Expected ']'")?;
                Ok(ChoiceDest::NestedChoices(items))
            }
            _ => Err(format!(
                "Expected choice destination, found {:?}",
                self.peek()
            )),
        }
    }

    fn parse_node_jump(&mut self) -> Result<NodeJump, String> {
        self.consume(&Token::Arrow, "Expected '->'")?;

        match self.peek() {
            Some(Token::Identifier(name)) => {
                let name = name.to_string();
                self.advance();
                Ok(NodeJump::Identifier(name))
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
                self.peek()
            )),
        }
    }

    fn parse_function_decl(&mut self) -> Result<FunctionDecl, String> {
        self.consume(&Token::Fn, "Expected 'fn'")?;

        let name = if let Some(Token::Identifier(name)) = self.advance() {
            name.to_string()
        } else {
            return Err("Expected function name".to_string());
        };

        self.consume(&Token::LeftParen, "Expected '('")?;

        let mut params = Vec::new();

        while !self.check(&Token::RightParen) && !self.is_at_end() {
            params.push(self.parse_param()?);

            if self.check(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        self.consume(&Token::RightParen, "Expected ')'")?;

        let return_type = if self.check(&Token::Arrow) {
            self.advance(); // consume '->'
            if let Some(Token::Identifier(type_name)) = self.advance() {
                Some(type_name.to_string())
            } else {
                return Err("Expected type after '->'".to_string());
            }
        } else {
            None
        };

        Ok(FunctionDecl {
            name,
            params,
            return_type,
        })
    }

    fn parse_param(&mut self) -> Result<Param, String> {
        let name = if let Some(Token::Identifier(name)) = self.advance() {
            name.to_string()
        } else {
            return Err("Expected parameter name".to_string());
        };

        self.consume(&Token::Colon, "Expected ':'")?;

        let type_name = if let Some(Token::Identifier(type_name)) = self.advance() {
            type_name.to_string()
        } else {
            return Err("Expected parameter type".to_string());
        };

        Ok(Param { name, type_name })
    }

    fn parse_func_call(&mut self) -> Result<FuncCall, String> {
        let name = if let Some(Token::Identifier(name)) = self.advance() {
            name.to_string()
        } else {
            return Err("Expected function name".to_string());
        };

        self.consume(&Token::LeftParen, "Expected '('")?;

        let mut args = Vec::new();

        while !self.check(&Token::RightParen) && !self.is_at_end() {
            args.push(self.parse_arg()?);

            if self.check(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        self.consume(&Token::RightParen, "Expected ')'")?;

        Ok(FuncCall { name, args })
    }

    fn parse_arg(&mut self) -> Result<Arg, String> {
        match self.peek() {
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
                if self.tokens.get(self.current + 1) == Some(&Token::LeftParen) {
                    Ok(Arg::FuncCall(Box::new(self.parse_func_call()?)))
                } else {
                    let name = name.to_string();
                    self.advance();
                    Ok(Arg::Identifier(name))
                }
            }
            _ => Err(format!("Expected argument, found {:?}", self.peek())),
        }
    }
}
