use crate::{datatypes::DataType, lexer::TokenKind, operators::Operator};
use colored::Colorize;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    UnknownTokenError(char),
    EndOfSourceCodeError,
    NoTokensAvailable,
    UnExpectedTokenError(TokenKind, TokenKind),
    CustomError(String),
    InvalidBinaryOperationError(DataType, Operator, DataType),
    InvalidUnaryOperationError(TokenKind),
    UnExpectedOperatorTokenError(TokenKind),
    UnknownKeywordError(String),
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            CompilerError::UnknownTokenError(token) => format!("Unknown token {token}"),
            CompilerError::EndOfSourceCodeError => {
                "reaced end of source code. unhandled error".to_string()
            }
            CompilerError::NoTokensAvailable => "No tokens available".to_string(),
            CompilerError::UnExpectedTokenError(expected, got) => {
                format!("unexpected token {:?}, expected {:?}", got, expected)
            }
            CompilerError::CustomError(msg) => msg.to_string(),
            CompilerError::InvalidBinaryOperationError(a, operator, b) => format!(
                "Invalid Binary Operation: cannot perform {:?} between {:?} and {:?}",
                operator, a, b
            ),
            CompilerError::UnExpectedOperatorTokenError(token) => {
                format!("Unexpected operator: {:?}", token)
            }
            CompilerError::InvalidUnaryOperationError(token) => {
                format!("Invalid unery operator: {:?}", token)
            }
            CompilerError::UnknownKeywordError(word) => format!("Unknown keyword: {word}"),
        };
        write!(f, "{}", text.red())
    }
}
