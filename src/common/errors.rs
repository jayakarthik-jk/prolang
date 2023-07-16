use std::fmt::Display;

use colored::Colorize;

use crate::common::operators::Operator;
use crate::lexical_analysis::lexer::TokenKind;

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    EndOfSourceCodeError,
    NoTokensAvailable,

    // Lexical Errors
    InvalidCharacter(char, usize, usize),
    InvalidNumber(String, usize, usize),
    UnterminatedString(usize, usize),

    // Syntax Errors
    UnexpectedToken(TokenKind, usize, usize),
    UnexpectedTokenWithExpected(TokenKind, TokenKind, usize, usize),

    // Evaluation Errors
    InvalidTokenAsBinaryOperator(TokenKind),
    InvalidOperatorForBinaryOperation(Operator),
    INvalidOperatorForUnaryOperation(Operator),
    InvalidTokenAsUnaryOperator(TokenKind),
    InvalidTokenAsLiteral(TokenKind),
    InvalidTokenAsOperator(TokenKind),

    // Semantic Errors
    UndefinedVariable(String),
    InvalidTokenAsIdentifier(crate::lexical_analysis::lexer::TokenKind),
    InvalidExpressionAssignment(usize, usize),
    InvalidAssignment,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            CompilerError::EndOfSourceCodeError => "Incomplete Expression".to_string(),
            CompilerError::NoTokensAvailable => "No tokens available".to_string(),
            CompilerError::InvalidCharacter(character, line, column) => format!(
                "Invalid character '{}' at line {}, column {}",
                character, line, column
            ),
            CompilerError::InvalidNumber(number, line, column) => format!(
                "Invalid number '{}' at line {}, column {}",
                number, line, column
            ),
            CompilerError::UnterminatedString(line, column) => {
                format!("Unterminated string at line {}, column {}", line, column)
            }
            CompilerError::UnexpectedToken(token, line, column) => format!(
                "Unexpected token '{}' at line {}, column {}",
                token, line, column
            ),
            CompilerError::UnexpectedTokenWithExpected(token, expected, line, column) => format!(
                "Unexpected token '{}' at line {}, column {}. Expected {}",
                token, line, column, expected
            ),
            CompilerError::InvalidTokenAsBinaryOperator(token) => {
                format!("Invalid token '{}' as binary operator", token)
            }
            CompilerError::InvalidOperatorForBinaryOperation(operator) => {
                format!("Invalid operator '{}' for binary operation", operator)
            }
            CompilerError::INvalidOperatorForUnaryOperation(operator) => {
                format!("Invalid operator '{}' for unary operation", operator)
            }
            CompilerError::InvalidTokenAsUnaryOperator(token) => {
                format!("Invalid token '{}' as unary operator", token)
            }
            CompilerError::InvalidTokenAsLiteral(token) => {
                format!("Invalid token '{}' as literal", token)
            }
            CompilerError::InvalidTokenAsOperator(token) => {
                format!("Invalid token '{}' as operator", token)
            }
            CompilerError::UndefinedVariable(name) => format!("Undefined variable '{}'", name),
            CompilerError::InvalidTokenAsIdentifier(token) => {
                format!("Invalid token '{}' as identifier", token)
            }
            CompilerError::InvalidExpressionAssignment(line, column) => format!(
                "Invalid expression assignment at line {}, column {}",
                line, column
            ),
            CompilerError::InvalidAssignment => "Invalid assignment".to_string(),
        };
        write!(f, "{}", text.red())
    }
}
