use super::Parser;
use super::error::ParseError;
use crate::ast::{
    BranchValue, ConstDecl, EnumDef, EventDef, FunctionDecl, NodeDef, NodeJump, Param, Program,
    TimelineDef, TimelineStmt, TopLevel, VarDecl, VarValue,
};
use crate::parser::expression::ExpressionParser;
use crate::parser::statement::StatementParser;
use crate::token::Token;

pub trait TopLevelParser {
    fn parse_program(&mut self) -> Result<Program, ParseError>;
    fn parse_top_level(&mut self) -> Result<TopLevel, ParseError>;

    fn parse_node_def(&mut self) -> Result<NodeDef, ParseError>;
    fn parse_node_jump(&mut self) -> Result<NodeJump, ParseError>;

    fn parse_function_decl(&mut self) -> Result<FunctionDecl, ParseError>;
    fn parse_param(&mut self) -> Result<Param, ParseError>;

    fn parse_var_decl(&mut self) -> Result<VarDecl, ParseError>;
    fn parse_const_decl(&mut self) -> Result<ConstDecl, ParseError>;

    fn parse_enum_def(&mut self) -> Result<EnumDef, ParseError>;
    fn parse_event_def(&mut self) -> Result<EventDef, ParseError>;

    fn parse_timeline_def(&mut self) -> Result<TimelineDef, ParseError>;
    fn parse_timeline_stmt(&mut self) -> Result<TimelineStmt, ParseError>;
}

impl<'a> TopLevelParser for Parser<'a> {
    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut body = Vec::new();

        while !self.is_at_end() {
            self.skip_comments_and_separators();

            if !self.is_at_end() {
                match self.parse_top_level() {
                    Ok(stmt) => body.push(stmt),
                    Err(err) => {
                        // Capture the span of the error token (approximately)
                        let span = self.get_current_span().unwrap_or((0, 0));
                        self.errors.push((err, span));
                        self.synchronize();
                    }
                }
            }
        }

        Ok(Program { body })
    }

    fn parse_top_level(&mut self) -> Result<TopLevel, ParseError> {
        self.skip_comments_and_separators();

        match self.peek().map(|t| &t.token) {
            Some(Token::Node) => Ok(TopLevel::NodeDef(self.parse_node_def()?)),
            Some(Token::Fn) => Ok(TopLevel::FunctionDecl(self.parse_function_decl()?)),
            Some(Token::Let) => Ok(TopLevel::VarDecl(self.parse_var_decl()?)),
            Some(Token::Const) | Some(Token::Pub) => {
                Ok(TopLevel::ConstDecl(self.parse_const_decl()?))
            }
            Some(Token::Enum) => Ok(TopLevel::EnumDef(self.parse_enum_def()?)),
            Some(Token::Event) => Ok(TopLevel::EventDef(self.parse_event_def()?)),
            Some(Token::Timeline) => Ok(TopLevel::TimelineDef(self.parse_timeline_def()?)),
            _ => Err(ParseError::UnexpectedToken {
                expected: "'node', 'fn', 'let', 'const', 'pub', 'enum', 'event', or 'timeline'"
                    .to_string(),
                found: self
                    .peek()
                    .map(|t| format!("{}", t.token))
                    .unwrap_or_else(|| "EOF".to_string()),
            }),
        }
    }

    fn parse_node_def(&mut self) -> Result<NodeDef, ParseError> {
        self.consume(&Token::Node, "Expected 'node'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err(ParseError::Custom(
                    "Expected identifier after 'node'".to_string(),
                ));
            }
        } else {
            return Err(ParseError::Custom(
                "Expected identifier after 'node'".to_string(),
            ));
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

        Ok(NodeDef {
            name,
            name_span,
            body,
            jump,
        })
    }

    fn parse_node_jump(&mut self) -> Result<NodeJump, ParseError> {
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
            _ => Err(ParseError::UnexpectedToken {
                expected: "identifier, 'return', or 'break'".to_string(),
                found: self
                    .peek()
                    .map(|t| format!("{}", t.token))
                    .unwrap_or_else(|| "EOF".to_string()),
            }),
        }
    }

    fn parse_function_decl(&mut self) -> Result<FunctionDecl, ParseError> {
        self.consume(&Token::Fn, "Expected 'fn'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err(ParseError::Custom("Expected function name".to_string()));
            }
        } else {
            return Err(ParseError::Custom("Expected function name".to_string()));
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

    fn parse_param(&mut self) -> Result<Param, ParseError> {
        let name = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                name.to_string()
            } else {
                return Err(ParseError::Custom("Expected parameter name".to_string()));
            }
        } else {
            return Err(ParseError::Custom("Expected parameter name".to_string()));
        };

        self.consume(&Token::Colon, "Expected ':'")?;

        let type_name = self.parse_type()?;

        Ok(Param { name, type_name })
    }

    fn parse_var_decl(&mut self) -> Result<VarDecl, ParseError> {
        self.consume(&Token::Let, "Expected 'let'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err(ParseError::ExpectedIdentifier {
                    found: format!("{}", token_info.token),
                });
            }
        } else {
            return Err(ParseError::UnexpectedEOF);
        };

        self.consume(&Token::Colon, "Expected ':' after variable name")?;

        // Check if this is a branch type variable
        if self.check(&Token::Branch) {
            self.advance(); // consume 'branch'

            // Check for optional enum type: branch<EnumType>
            let enum_type = if self.check(&Token::Less) {
                self.advance(); // consume <
                let type_token = self.consume_identifier("Expected enum type or variable name")?;
                self.consume(&Token::Greater, "Expected '>' after type")?;
                Some(type_token)
            } else {
                None
            };

            // Parse branch cases in brackets: [condition, text, ...]
            self.consume(&Token::LeftBracket, "Expected '[' to start branch cases")?;

            let mut cases = Vec::new();

            while !self.check(&Token::RightBracket) && !self.is_at_end() {
                let case = self.parse_branch_case()?;
                cases.push(case);

                // Cases can be separated by newlines or commas (optional)
                if self.check(&Token::Comma) {
                    self.advance();
                }
            }

            self.consume(&Token::RightBracket, "Expected ']' to end branch cases")?;

            Ok(VarDecl {
                name,
                name_span,
                type_name: "Branch".to_string(),
                value: Some(VarValue::Branch(BranchValue { enum_type, cases })),
            })
        } else {
            // Regular variable declaration
            let type_name = self.parse_type()?;

            let value = if self.check(&Token::Equals) {
                self.advance(); // consume '='
                Some(self.parse_var_value()?)
            } else {
                None
            };

            Ok(VarDecl {
                name,
                name_span,
                type_name,
                value,
            })
        }
    }

    fn parse_const_decl(&mut self) -> Result<ConstDecl, ParseError> {
        let is_public = if self.check(&Token::Pub) {
            self.advance();
            true
        } else {
            false
        };

        self.consume(&Token::Const, "Expected 'const'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err(ParseError::Custom("Expected constant name".to_string()));
            }
        } else {
            return Err(ParseError::Custom("Expected constant name".to_string()));
        };

        self.consume(&Token::Colon, "Expected ':' after constant name")?;

        let type_name = self.parse_type()?;

        self.consume(&Token::Equals, "Expected '=' after constant type")?;

        let value = self.parse_var_value()?;

        Ok(ConstDecl {
            is_public,
            name,
            name_span,
            type_name,
            value,
        })
    }

    fn parse_enum_def(&mut self) -> Result<EnumDef, ParseError> {
        self.consume(&Token::Enum, "Expected 'enum'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err(ParseError::Custom("Expected enum name".to_string()));
            }
        } else {
            return Err(ParseError::Custom("Expected enum name".to_string()));
        };

        self.consume(&Token::LeftBrace, "Expected '{'")?;

        let mut variants = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if self.check(&Token::RightBrace) {
                break;
            }

            if let Some(token_info) = self.advance() {
                if let Token::Identifier(variant) = &token_info.token {
                    variants.push(variant.to_string());
                } else {
                    return Err(ParseError::Custom("Expected enum variant name".to_string()));
                }
            } else {
                return Err(ParseError::Custom("Expected enum variant name".to_string()));
            }

            self.skip_optional_separators();
        }

        self.consume(&Token::RightBrace, "Expected '}'")?;

        Ok(EnumDef {
            name,
            name_span,
            variants,
        })
    }

    fn parse_event_def(&mut self) -> Result<EventDef, ParseError> {
        self.consume(&Token::Event, "Expected 'event'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err(ParseError::Custom(
                    "Expected identifier after 'event'".to_string(),
                ));
            }
        } else {
            return Err(ParseError::Custom(
                "Expected identifier after 'event'".to_string(),
            ));
        };

        self.consume(&Token::LeftBrace, "Expected '{'")?;

        let mut index = None;
        let mut action = None;
        let mut duration = None;

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if self.check(&Token::RightBrace) {
                break;
            }

            match self.peek().map(|t| &t.token) {
                Some(Token::Index) => {
                    self.advance();
                    self.consume(&Token::Colon, "Expected ':' after 'index'")?;
                    if let Some(Token::Number(n)) = self.peek().map(|t| &t.token) {
                        index = Some(
                            n.parse::<f64>()
                                .map_err(|_| ParseError::InvalidNumber(n.to_string()))?,
                        );
                        self.advance();
                    } else {
                        return Err(ParseError::Custom(
                            "Expected number after 'index:'".to_string(),
                        ));
                    }
                }
                Some(Token::Action) => {
                    self.advance();
                    self.consume(&Token::Colon, "Expected ':' after 'action'")?;
                    action = Some(self.parse_event_action()?);
                }
                Some(Token::Duration) => {
                    self.advance();
                    self.consume(&Token::Colon, "Expected ':' after 'duration'")?;
                    if let Some(Token::Number(n)) = self.peek().map(|t| &t.token) {
                        duration = Some(
                            n.parse::<f64>()
                                .map_err(|_| ParseError::InvalidNumber(n.to_string()))?,
                        );
                        self.advance();
                    } else {
                        return Err(ParseError::Custom(
                            "Expected number after 'duration:'".to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "'index', 'action', or 'duration'".to_string(),
                        found: self
                            .peek()
                            .map(|t| format!("{}", t.token))
                            .unwrap_or_else(|| "EOF".to_string()),
                    });
                }
            }

            self.skip_optional_separators();
        }

        self.consume(&Token::RightBrace, "Expected '}'")?;

        let action = action.ok_or(ParseError::Custom(
            "Event definition must have an 'action' field".to_string(),
        ))?;

        Ok(EventDef {
            name,
            name_span,
            index,
            action,
            duration,
        })
    }

    fn parse_timeline_def(&mut self) -> Result<TimelineDef, ParseError> {
        self.consume(&Token::Timeline, "Expected 'timeline'")?;

        let (name, name_span) = if let Some(token_info) = self.advance() {
            if let Token::Identifier(name) = &token_info.token {
                (name.to_string(), Some((token_info.start, token_info.end)))
            } else {
                return Err(ParseError::Custom(
                    "Expected identifier after 'timeline'".to_string(),
                ));
            }
        } else {
            return Err(ParseError::Custom(
                "Expected identifier after 'timeline'".to_string(),
            ));
        };

        self.consume(&Token::LeftBrace, "Expected '{'")?;

        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.skip_comments_and_separators();

            if !self.check(&Token::RightBrace) && !self.is_at_end() {
                body.push(self.parse_timeline_stmt()?);
                self.skip_optional_separators();
            }
        }

        self.consume(&Token::RightBrace, "Expected '}'")?;

        Ok(TimelineDef {
            name,
            name_span,
            body,
        })
    }

    fn parse_timeline_stmt(&mut self) -> Result<TimelineStmt, ParseError> {
        match self.peek().map(|t| &t.token) {
            Some(Token::Run) => Ok(TimelineStmt::Run(self.parse_run_stmt()?)),
            Some(Token::Now) => {
                // Parse "now run EventName" - ignores duration
                self.advance(); // consume "now"
                if !self.check(&Token::Run) {
                    return Err(ParseError::Custom("Expected 'run' after 'now'".to_string()));
                }
                let mut run_stmt = self.parse_run_stmt()?;
                run_stmt.ignore_duration = true;
                Ok(TimelineStmt::Run(run_stmt))
            }
            Some(Token::Wait) => {
                self.advance();
                if let Some(Token::Number(n)) = self.peek().map(|t| &t.token) {
                    let duration = n
                        .parse::<f64>()
                        .map_err(|_| ParseError::InvalidNumber(n.to_string()))?;
                    self.advance();
                    Ok(TimelineStmt::Wait(duration))
                } else {
                    Err(ParseError::Custom(
                        "Expected number after 'wait'".to_string(),
                    ))
                }
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "'run', 'now', or 'wait'".to_string(),
                found: self
                    .peek()
                    .map(|t| format!("{}", t.token))
                    .unwrap_or_else(|| "EOF".to_string()),
            }),
        }
    }
}
