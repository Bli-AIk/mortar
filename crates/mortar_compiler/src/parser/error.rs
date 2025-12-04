use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: String },
    ExpectedIdentifier { found: String },
    ExpectedString { found: String },
    UnexpectedEOF,
    InvalidNumber(String),
    Custom(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "Expected {}, found {}", expected, found)
            }
            ParseError::ExpectedIdentifier { found } => {
                write!(f, "Expected identifier, found {}", found)
            }
            ParseError::ExpectedString { found } => {
                write!(f, "Expected string, found {}", found)
            }
            ParseError::UnexpectedEOF => write!(f, "Unexpected end of input"),
            ParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ParseError::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for ParseError {}
