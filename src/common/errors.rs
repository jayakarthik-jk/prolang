use std::fmt::Display;

use colored::Colorize;

use crate::common::operators::Operator;
use crate::lexical_analysis::token::TokenKind;

use super::datatypes::Variable;

#[derive(Debug, PartialEq, Clone)]
pub enum CompilerError {
    NoTokensAvailable,

    // Lexical Errors
    InvalidCharacter(char, usize, usize),
    InvalidNumber(String, usize, usize),
    UnterminatedString(usize, usize),
    InvalidKeyword,
    InvalidUtf8Character,

    // Syntax Errors
    UnexpectedToken(TokenKind, usize, usize),
    InvalidOperationAsAssignmentOperation,
    CannotConvertFromImmutableToMutable,
    UnInitializedVariable(String),

    // Evaluation Errors
    InvalidOperatorForBinaryOperation(Operator),
    InvalidOperatorForUnaryOperation(Operator),

    // Semantic Errors
    UndefinedVariable(String),
    InvalidExpressionAssignment,
    InvalidAssignment,
    InvalidStringParsing(Variable),
    InvalidUneryOperation,
    UnsupportedOperationBetween(Variable, Operator, Variable),
    MathUndefined,
    OperationOnUndefined,
    InvalidUseOfMutableKeyword,
    ImmutableVariable(String),

    // warnings
    Warnings(&'static str),
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
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
            CompilerError::InvalidOperatorForBinaryOperation(operator) => {
                format!("Invalid operator '{}' for binary operation", operator)
            }
            CompilerError::InvalidOperatorForUnaryOperation(operator) => {
                format!("Invalid operator '{}' for unary operation", operator)
            }
            CompilerError::UndefinedVariable(name) => format!("Undefined variable '{}'", name),
            CompilerError::InvalidExpressionAssignment => {
                "Invalid expression assignment".to_string()
            }
            CompilerError::InvalidAssignment => "Invalid assignment".to_string(),
            CompilerError::InvalidKeyword => "Invalid keyword".to_string(),
            CompilerError::InvalidStringParsing(a) => {
                format!("Invalid string parsing: '{}' is not a valid Nubmer", a)
            }
            CompilerError::InvalidUneryOperation => "Invalid unary operation".to_string(),
            CompilerError::UnsupportedOperationBetween(left, operator, right) => {
                format!(
                    "Unsupported operation {} between '{}' and '{}'",
                    operator, left, right
                )
            }
            CompilerError::MathUndefined => "Math Error: undefined".to_string(),
            CompilerError::OperationOnUndefined => {
                "Cannot perform operation on Undefined".to_string()
            }
            CompilerError::InvalidUtf8Character => "Invalid UTF-8 character".to_string(),
            CompilerError::InvalidUseOfMutableKeyword => {
                "Invalid use of `mutable` keyword".to_string()
            }
            CompilerError::ImmutableVariable(name) => {
                format!("cannot mutate Immutable variable '{}'", name)
            }
            CompilerError::InvalidOperationAsAssignmentOperation => {
                "Invalid operation as assignment operation".to_string()
            }
            CompilerError::CannotConvertFromImmutableToMutable => {
                "Cannot convert from Immutable to Mutable".to_string()
            }
            CompilerError::Warnings(warning) => warning.to_string(),
            CompilerError::UnInitializedVariable(name) => format!("Uninitialized variable {name}"),
        };
        write!(f, "{}", text.red())
    }
}
