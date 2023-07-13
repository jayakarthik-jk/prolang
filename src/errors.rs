use crate::lexer::TokenKind;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum LexerError {
    UnknownTokenError(char),
    EndOfSourceCodeError,
    NoTokensAvailable,
    /// expected, unexpected
    UnExpectedToken(TokenKind, TokenKind),
    CustomError(String),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            LexerError::UnknownTokenError(token) => format!("Unknown token {token}"),
            LexerError::EndOfSourceCodeError => {
                "reaced end of source code. unhandled error".to_string()
            }
            LexerError::NoTokensAvailable => "No tokens available".to_string(),
            LexerError::UnExpectedToken(expected, got) => {
                format!("unexpected token {:?}, expected {:?}", got, expected)
            }
            LexerError::CustomError(msg) => msg.to_string(),
        };
        write!(f, "{text}")
    }
}
